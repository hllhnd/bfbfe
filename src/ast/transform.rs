use thiserror::Error;

use super::Element;
use super::Node;
use crate::token::Token;

#[derive(Debug, Eq, Error, PartialEq)]
#[non_exhaustive]
pub enum ASTError
{
    #[error("JumpForward at position {0} has no matching JumpBackward")]
    UnmatchedJumpForward(usize),
    #[error("JumpBackward at position {0} has no matching JumpForward")]
    UnmatchedJumpBackward(usize),
}

pub fn transform(tks: &[Token]) -> Result<Node, ASTError>
{
    let mut node_list: Vec<Node> = Vec::new();

    // Initial node. This will be the node that contains all the child nodes when
    // parsing is done.
    node_list.push(Node {
        content: Vec::new()
    });

    let mut first_jfw: Option<usize> = None;

    for (i, tk) in tks.iter().enumerate() {
        let lst = node_list.last_mut().unwrap();

        match tk {
            Token::IncrementPointer => {
                lst.content.push(Element::MovPtr {
                    by: 1
                });
            }

            Token::DecrementPointer => {
                lst.content.push(Element::MovPtr {
                    by: -1
                });
            }

            Token::IncrementValue => {
                lst.content.push(Element::MutVal {
                    at: 0, by: 1
                });
            }

            Token::DecrementValue => {
                lst.content.push(Element::MutVal {
                    at: 0, by: -1
                });
            }

            Token::OutputByte => {
                lst.content.push(Element::Push {
                    from: 0
                });
            }

            Token::ReadByte => {
                lst.content.push(Element::Read {
                    to: 0
                });
            }

            Token::JumpForward => {
                if first_jfw.is_none() {
                    first_jfw = Some(i);
                }

                // Begin new node
                node_list.push(Node {
                    content: Vec::new()
                });
            }

            Token::JumpBackward => {
                // If this condition holds true, it probably means a ] was passed without a [
                // preceding it.
                if node_list.len() < 2 {
                    return Err(ASTError::UnmatchedJumpBackward(i));
                }

                let child_node = node_list.pop().unwrap();
                let parent_node = node_list.last_mut().unwrap();

                parent_node.content.push(Element::CondBlck {
                    node: child_node
                });
            }
        }
    }

    // Likewise, if this is true it probably means a [ was passed without a matching
    // ] after it.
    if node_list.len() != 1 {
        return Err(ASTError::UnmatchedJumpForward(first_jfw.unwrap()));
    }

    Ok(node_list[0].clone())
}

#[cfg(test)]
#[allow(clippy::enum_glob_use)]
mod tests
{
    use Element::*;
    use Token::*;

    use super::*;

    #[test]
    fn transform_success()
    {
        const TOKENS: [Token; 10] = [
            IncrementPointer,
            DecrementPointer,
            IncrementValue,
            DecrementValue,
            OutputByte,
            ReadByte,
            JumpForward,
            IncrementPointer,
            IncrementValue,
            JumpBackward,
        ];

        assert_eq!(
            transform(&TOKENS),
            Ok(Node {
                content: vec![
                    MovPtr {
                        by: 1
                    },
                    MovPtr {
                        by: -1
                    },
                    MutVal {
                        at: 0, by: 1
                    },
                    MutVal {
                        at: 0, by: -1
                    },
                    Push {
                        from: 0
                    },
                    Read {
                        to: 0
                    },
                    CondBlck {
                        node: Node {
                            content: vec![
                                MovPtr {
                                    by: 1
                                },
                                MutVal {
                                    at: 0, by: 1
                                }
                            ],
                        },
                    }
                ],
            })
        );
    }

    #[test]
    fn transform_unmatched_jfw()
    {
        const TOKENS: [Token; 7] = [
            IncrementPointer,
            DecrementPointer,
            IncrementValue,
            JumpForward, // Here's the bad egg
            DecrementValue,
            OutputByte,
            ReadByte,
        ];

        assert_eq!(transform(&TOKENS), Err(ASTError::UnmatchedJumpForward(3)));
    }

    #[test]
    fn transform_unmatched_jbw()
    {
        const TOKENS: [Token; 7] = [
            IncrementPointer,
            DecrementPointer,
            IncrementValue,
            JumpBackward, // This time it's a backwards jump
            DecrementValue,
            OutputByte,
            ReadByte,
        ];

        assert_eq!(transform(&TOKENS), Err(ASTError::UnmatchedJumpBackward(3)));
    }
}
