use thiserror::Error;

/// Represents the eight Brainfuck instructions.
#[derive(Debug, Eq, PartialEq)]
pub enum Token
{
    /// Corresponds to the Brainfuck instruction `>`.
    ///
    /// In Brainfuck, this moves the current position on the tape forward by
    /// one.
    IncrementPointer,
    /// Corresponds to the Brainfuck instruction `<`.
    ///
    /// In Brainfuck, this moves the current position on the tape back by one.
    DecrementPointer,
    /// Corresponds to the Brainfuck instruction `+`.
    ///
    /// In Brainfuck, this increments the current value on the tape by one.
    IncrementValue,
    /// Corresponds to the Brainfuck instruction `-`.
    ///
    /// In Brainfuck, this decrements the current value on the tape by one.
    DecrementValue,
    /// Corresponds to the Brainfuck instruction `.`.
    ///
    /// In Brainfuck, this outputs the current byte to the standard output as a
    /// character.
    OutputByte,
    /// Corresponds to the Brainfuck instruction `,`.
    ///
    /// In Brainfuck, this reads a single byte from the standard input into the
    /// current value on the tape.
    ReadByte,
    /// Corresponds to the Brainfuck instruction `[`.
    ///
    /// In Brainfuck, this checks if the current value is zero, and skips to the
    /// instruction after its matching `]` instruction if it is. Otherwise, it
    /// does nothing.
    JumpForward,
    /// Corresponds to the Brainfuck instruction `]`.
    ///
    /// In Brainfuck, this checks if the current value is nonzero, and jumps
    /// backwards to its matching `[` instruction if it is. Otherwise, it does
    /// nothing.
    JumpBackward,
}

#[derive(Debug, Eq, Error, PartialEq)]
#[non_exhaustive]
pub enum TokenizingError
{
    #[error("invalid byte passed {0} at pos {1}")]
    UnknownByte(u8, usize),
}

/// Transforms a byte array into a [`Vec`] of [`Token`].
///
/// # Arguments
/// * `prog` - Brainfuck program represented as bytes
///
/// # Returns
/// A [`Vec`] of [`Token`] if successful, otherwise a [`TokenizingError`].
pub fn tokenize(prog: &[u8]) -> Result<Vec<Token>, TokenizingError>
{
    let mut tokens: Vec<Token> = Vec::with_capacity(prog.len());

    for (i, ch) in prog.iter().enumerate() {
        let tk = match ch {
            b'>' => Token::IncrementPointer,
            b'<' => Token::DecrementPointer,
            b'+' => Token::IncrementValue,
            b'-' => Token::DecrementValue,
            b'.' => Token::OutputByte,
            b',' => Token::ReadByte,
            b'[' => Token::JumpForward,
            b']' => Token::JumpBackward,
            _ => return Err(TokenizingError::UnknownByte(*ch, i)),
        };

        tokens.push(tk);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_tokenize_all()
    {
        assert_eq!(
            tokenize(b"><+-.,[]"),
            Ok(vec![
                Token::IncrementPointer,
                Token::DecrementPointer,
                Token::IncrementValue,
                Token::DecrementValue,
                Token::OutputByte,
                Token::ReadByte,
                Token::JumpForward,
                Token::JumpBackward,
            ])
        );
    }

    #[test]
    fn test_tokenize_invalid_byte()
    {
        assert_eq!(tokenize(b"><+-.,[]W"), Err(TokenizingError::UnknownByte(b'W', 8)));
    }
}
