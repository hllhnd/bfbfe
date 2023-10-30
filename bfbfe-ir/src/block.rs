use core::ops::Deref;
use core::ops::DerefMut;

use super::instruction::IRInstruction;
use crate::optimization::constants::ConstantsOptimization;
use crate::optimization::merge::MergeInstructions;
use crate::optimization::reorder::ReorderingOptimizations;

#[derive(Debug, Eq, PartialEq)]
pub struct IRBlock
{
    pub content: Vec<IRInstruction>,
}

impl Clone for IRBlock
{
    #[inline]
    fn clone(&self) -> Self
    {
        Self::with_instructions(self.content.clone())
    }
}

impl Deref for IRBlock
{
    type Target = Vec<IRInstruction>;

    #[inline]
    fn deref(&self) -> &Self::Target
    {
        &self.content
    }
}

impl DerefMut for IRBlock
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.content
    }
}

impl IRBlock
{
    #[inline]
    pub const fn new() -> Self
    {
        Self {
            content: Vec::new()
        }
    }

    #[inline]
    pub const fn with_instructions(vc: Vec<IRInstruction>) -> Self
    {
        Self {
            content: vc
        }
    }

    pub fn optimize(mut self) -> Self
    {
        for instr in &mut self.content {
            if let &mut IRInstruction::ConditionalBlock(ref mut block) = instr {
                *block = Box::new(block.clone().optimize());
            }
        }

        self.merge_instructions();
        // The find_set_to_zero optimization can cause an extremely rare misoptimization. Feel free to test and uncomment the following line if your program does not get broken.
        //self.find_set_to_zero();
        self.find_set_to_value();
        self.reorder_instructions();

        self
    }
}
