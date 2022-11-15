use super::IRElement;
use super::IRNode;
use crate::token::Token;

pub fn transform(tks: &[Token]) -> IRNode
{
    let mut node_list: Vec<IRNode> = Vec::new();

    // Initial node. This will be the node that contains all the child nodes when
    // parsing is done.
    node_list.push(IRNode {
        content: Vec::new()
    });

    for tk in tks {
        let lst = node_list.last_mut().unwrap();

        match tk {
            Token::IncrementPointer => {
                lst.push(IRElement::MovPtr {
                    by: 1
                });
            }

            Token::DecrementPointer => {
                lst.push(IRElement::MovPtr {
                    by: -1
                });
            }

            Token::IncrementValue => {
                lst.push(IRElement::MutVal {
                    at: 0, by: 1
                });
            }

            Token::DecrementValue => {
                lst.push(IRElement::MutVal {
                    at: 0, by: -1
                });
            }

            Token::OutputByte => {
                lst.push(IRElement::Push {
                    from: 0
                });
            }

            Token::ReadByte => {
                lst.push(IRElement::Read {
                    to: 0
                });
            }

            Token::JumpForward => {
                // Begin new node
                node_list.push(IRNode {
                    content: Vec::new()
                });
            }

            Token::JumpBackward => {
                // Store length as local for mutable borrow
                let len = node_list.len();

                // If this assertion fails, it probably means a ] was passed without a [
                // preceding it.
                assert!(len >= 2);

                let child_node = node_list.pop().unwrap();
                let parent_node = node_list.last_mut().unwrap();

                parent_node.push(IRElement::CondBlck {
                    node: child_node
                });
            }
        }
    }

    // Likewise, if this assertion fails it probably means a [ was passed without a
    // matching ] after it.
    assert!(node_list.len() == 1);

    node_list.get(0).unwrap().clone()
}

#[cfg(test)]
#[allow(clippy::enum_glob_use)]
mod tests
{
    use IRElement::*;
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
            IRNode {
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
                        node: IRNode {
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
            }
        );
    }

    #[test]
    #[should_panic]
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

        transform(&TOKENS);
    }

    #[test]
    #[should_panic]
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

        transform(&TOKENS);
    }
}
