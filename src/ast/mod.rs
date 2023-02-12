pub mod transform;

/// Contains an array of [`Element`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node
{
    pub content: Vec<Element>,
}

/// An element of BFBFE's AST
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Element
{
    /// Move the pointer by `by`.
    ///
    /// In pseudocode, this is equivalent to `ptr += by`.
    MovPtr
    {
        by: isize
    },

    /// Mutates the current value offset by `at` by `by`.
    ///
    /// In pseudocode, this is equivalent to `tape[ptr + at] += by`.
    MutVal
    {
        at: isize, by: isize
    },

    /// Read one byte from stdin to the value at the current pointer offset by
    /// `to`.
    ///
    /// In pseudocode, this is equivalent to `tape[ptr + at] = read_byte()`.
    Read
    {
        to: isize
    },

    /// Write one byte to stdout from the value at the current pointer offset by
    /// `from`.
    ///
    /// In pseudocode, this is equivalent to `print(tape[ptr + from])`.
    Push
    {
        from: isize
    },

    /// When reaching this instruction, the current value is checked for a zero
    /// value. If it is non-zero, `node`'s contents are executed. If it is zero,
    /// this instruction is skipped.
    ///
    /// When `node`'s contents are finished executing, a check is performed
    /// again for a zero value. If it is non-zero, `node` is executed again, and
    /// this repeats until the check finds the value to be zero.
    ///
    /// In pseudocode, this is equivalent to the following:
    /// ```pseudocode
    /// while (tape[ptr] != 0) {
    ///     // Run block contents
    /// }
    /// ```
    CondBlck
    {
        node: Node
    },
}
