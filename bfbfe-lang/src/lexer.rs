use itertools::Itertools;

use crate::token::Token;

pub fn tokenize_whole_program(program: &[char]) -> Vec<Token>
{
    program
        .iter()
        .filter_map(|ch| match ch {
            '>' => Some(Token::IncrementPointer),
            '<' => Some(Token::DecrementPointer),
            '+' => Some(Token::IncrementValue),
            '-' => Some(Token::DecrementValue),
            '.' => Some(Token::PushByte),
            ',' => Some(Token::ReadByte),
            '[' => Some(Token::JumpForward),
            ']' => Some(Token::JumpBackward),
            _ => None,
        })
        .collect_vec()
}
