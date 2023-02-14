pub mod transform;

/// Contains an array of [`Element`].
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Node
{
    pub content: Vec<Element>,
}

/// An element of BFBFE's AST
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Element
{
    /// Perform addition on the data pointer.
    PointerAdd(u16),
    /// Perform subtraction on the data pointer.
    PointerSub(u16),
    /// Set the current value to a constant.
    ValueSet(u8),
    /// Perform addition on the current value.
    ValueAdd(u8),
    /// Perform subtraction on the current value.
    ValueSub(u8),
    /// Send one byte from the current value to the output source. The current
    /// value is left unchanged.
    Push,
    /// Read one byte from the input source into the current value, overwriting
    /// it.
    Pull,
    /// Perform a check for a non-zero current value, repeating the code within
    /// while this holds true. Once this condition is false, or if it was never
    /// true to begin with, execution will continue on to the next element.
    ///
    /// Note that the condition is explicitly not necessarily the original value
    /// being checked; any modifications to the data pointer should result in
    /// that value being checked instead.
    Conditional(Node),
}
