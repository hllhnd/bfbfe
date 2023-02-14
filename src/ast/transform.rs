use thiserror::Error;

use super::Block;
use super::Element;
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

pub fn transform(tks: &[Token]) -> Result<Block, ASTError>
{
    // To avoid recursion, this buffer stores all the blocks while their contents
    // are being worked on. When a block is complete, it is inserted into the
    // previous block within this Vec.
    let mut block_buffer = vec![Block::new()];

    let mut first_jfw: Option<usize> = None;

    for (i, tk) in tks.iter().enumerate() {
        let current_block = block_buffer.last_mut().unwrap();

        match tk {
            Token::IncrementPointer => current_block.push(Element::PointerAdd(1)),
            Token::DecrementPointer => current_block.push(Element::PointerSub(1)),
            Token::IncrementValue => current_block.push(Element::ValueAdd(1)),
            Token::DecrementValue => current_block.push(Element::ValueSub(1)),
            Token::OutputByte => current_block.push(Element::Push),
            Token::ReadByte => current_block.push(Element::Pull),

            Token::JumpForward => {
                if first_jfw.is_none() {
                    first_jfw = Some(i);
                }

                // Begin new block
                block_buffer.push(Block::new());
            }

            Token::JumpBackward => {
                // If this condition holds true, it probably means a ] was passed without a [
                // preceding it.
                if block_buffer.len() < 2 {
                    return Err(ASTError::UnmatchedJumpBackward(i));
                }

                let current_block = block_buffer.pop().unwrap();
                block_buffer
                    .last_mut()
                    .unwrap()
                    .push(Element::Conditional(current_block));
            }
        }
    }

    // Likewise, if this is true it probably means a [ was passed without a matching
    // ] after it.
    if block_buffer.len() != 1 {
        return Err(ASTError::UnmatchedJumpForward(first_jfw.unwrap()));
    }

    Ok(block_buffer.pop().unwrap())
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
            Ok(vec![
                PointerAdd(1),
                PointerSub(1),
                ValueAdd(1),
                ValueSub(1),
                Push,
                Pull,
                Conditional(vec![PointerAdd(1), ValueAdd(1)])
            ]),
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
