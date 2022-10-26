/// The raw ingredients of a Brainfuck program. As is implied by the name,
/// `Token` bears no analytical information regarding the functionality,
/// structure, or data of a Brainfuck program other than which Brainfuck
/// instruction it represents.
///
/// `Token` maps 1:1 with the eight Brainfuck commands.
///
/// | Character | Name               |
/// |-----------|--------------------|
/// | `>`       | `IncrementPointer` |
/// | `<`       | `DecrementPointer` |
/// | `+`       | `IncrementValue`   |
/// | `-`       | `DecrementValue`   |
/// | `.`       | `PushByte`         |
/// | `,`       | `ReadByte`         |
/// | `[`       | `JumpForward`      |
/// | `]`       | `JumpBackward`     |
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token
{
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    PushByte,
    ReadByte,
    JumpForward,
    JumpBackward,
}
