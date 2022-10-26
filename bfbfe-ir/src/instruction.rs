use super::block::IRBlock;

/// Represents instructions for the BFBFE IR.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum IRInstruction
{
    // ~~~~~~ Meta Instructions ~~~~~~
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Marks the beginning of a program.
    /// There should only be one of this instruction in an entire program.
    BeginProgram,
    /// Marks the end of a program.
    /// There should only be one of this instruction in an entire program.
    EndProgram,

    // ~~~~~~~ Data Processing ~~~~~~~
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Traverse the current pointer by `val`.
    TraverseBy
    {
        val: isize
    },
    /// Mutate the value `pos` positions relative to the current value by `val`.
    MutateValue
    {
        pos: isize, val: isize
    },
    /// Set the value `pos` from the pointer to zero.
    SetTo
    {
        pos: isize, val: isize
    },

    // ~~~~~~~~~~~~~ I/O ~~~~~~~~~~~~~
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Output the bytes relative to the pointer by the positions in `poslst`.
    OutputBytes
    {
        poslst: Vec<isize>
    },
    /// Read bytes into the values relative to the pointer by the positions in
    /// `poslst`. The amount of bytes that will be read is equal to the length
    /// of `poslst`.
    ReadBytes
    {
        poslst: Vec<isize>
    },

    // ~~~~~~~~~~~ Blocks ~~~~~~~~~~~~
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// A block of instructions to be executed while the currently pointed to
    /// byte is non-zero.
    ConditionalBlock(Box<IRBlock>),
}
