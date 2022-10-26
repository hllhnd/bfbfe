#![allow(clippy::std_instead_of_core)]

use std::cmp::Ordering;

use bfbfe_ir::block::IRBlock;
use bfbfe_ir::instruction::IRInstruction;
use thiserror::Error;

use crate::token::Token;

#[derive(Debug, Error)]
pub enum InstructionizingError
{
    #[error("unmatched JumpForward")]
    UnmatchedJumpForward,
    #[error("unmatched JumpBackward")]
    UnmatchedJumpBackward,
}

/// Transforms an Iterator of Token into a single [`IRBlock`]
pub fn instructionize(tokens: &[Token]) -> Result<IRBlock, InstructionizingError>
{
    let it = tokens.iter();
    match Ord::cmp(
        &it.clone().filter(|tk| **tk == Token::JumpForward).count(),
        &it.filter(|tk| **tk == Token::JumpBackward).count(),
    ) {
        Ordering::Greater => {
            return Err(InstructionizingError::UnmatchedJumpForward);
        }

        Ordering::Less => {
            return Err(InstructionizingError::UnmatchedJumpBackward);
        }

        Ordering::Equal => {}
    }

    let mut instrs: Vec<IRInstruction> = Vec::new();
    instrs.push(IRInstruction::BeginProgram);
    instrs.extend(_instructionize(tokens));
    instrs.push(IRInstruction::EndProgram);
    Ok(IRBlock::with_instructions(instrs))
}

fn _instructionize(tokens: &[Token]) -> Vec<IRInstruction>
{
    let mut content: Vec<IRInstruction> = Vec::new();

    let mut it = tokens.iter();
    while let Some(tk) = it.next() {
        match tk {
            Token::IncrementPointer => {
                content.push(IRInstruction::TraverseBy {
                    val: 1
                });
            }

            Token::DecrementPointer => {
                content.push(IRInstruction::TraverseBy {
                    val: -1
                });
            }

            Token::IncrementValue => {
                content.push(IRInstruction::MutateValue {
                    pos: 0, val: 1
                });
            }

            Token::DecrementValue => {
                content.push(IRInstruction::MutateValue {
                    pos: 0, val: -1
                });
            }

            Token::PushByte => {
                content.push(IRInstruction::OutputBytes {
                    poslst: [0].to_vec()
                });
            }

            Token::ReadByte => {
                content.push(IRInstruction::ReadBytes {
                    poslst: [0].to_vec()
                });
            }

            Token::JumpForward => {
                let res = {
                    let mut new_tokens: Vec<Token> = Vec::new();
                    let mut depth = 1_usize;

                    for tk in it.by_ref() {
                        match tk {
                            Token::JumpForward => {
                                depth += 1;
                            }

                            Token::JumpBackward => {
                                depth -= 1;
                            }

                            _ => {}
                        }

                        new_tokens.push(tk.clone());

                        if depth == 0 {
                            break;
                        }
                    }

                    IRInstruction::ConditionalBlock(Box::new(IRBlock::with_instructions(_instructionize(&new_tokens))))
                };

                content.push(res);
            }

            Token::JumpBackward => {}
        };
    }

    content
}
