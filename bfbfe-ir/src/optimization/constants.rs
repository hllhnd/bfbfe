use crate::block::IRBlock;
use crate::instruction::IRInstruction;

pub trait ConstantsOptimization
{
    fn find_set_to_zero(&mut self);
    fn find_set_to_value(&mut self);
}

impl ConstantsOptimization for IRBlock
{
    /// Creates `SetTo` instructions by looking for the pattern of a single
    /// `MutateValue` inside a `ConditionalBlock`.
    ///
    /// It catches any block like this unless the `MutateValue`:
    ///     * has a value of 0 (always infinite loop)
    ///     * is a divisor of 256 (sometimes infinite loop)
    fn find_set_to_zero(&mut self)
    {
        let mut buffer: Vec<IRInstruction> = Vec::new();

        'a: for instr in &self.content {
            if let IRInstruction::ConditionalBlock(block) = &instr {
                if block.len() == 1 {
                    if let IRInstruction::MutateValue {
                        pos,
                        val,
                    } = {
                        // SAFETY: The length was just verified to be 1
                        unsafe { block.get_unchecked(0) }
                    } {
                        if *pos != 0 {
                            continue 'a;
                        }

                        let val = val.abs();
                        if (val != 0 && 256 % val != 0) || val == 1 {
                            buffer.push(IRInstruction::SetTo {
                                pos: 0, val: 0
                            });

                            continue 'a;
                        }
                    }
                }
            }

            buffer.push(instr.clone());
        }

        self.content = buffer;
    }

    fn find_set_to_value(&mut self)
    {
        let mut buffer: Vec<IRInstruction> = Vec::new();

        let it = self.iter();
        let mut pk = it.peekable();
        while let Some(instr) = pk.next() {
            if let IRInstruction::SetTo {
                pos: set_pos,
                val: set_val,
            } = instr
            {
                if let Some(IRInstruction::MutateValue {
                    pos: mut_pos,
                    val: mut_val,
                }) = pk.peek()
                {
                    if set_pos != mut_pos {
                        continue;
                    }

                    buffer.push(IRInstruction::SetTo {
                        pos: *set_pos,
                        val: set_val + mut_val,
                    });

                    pk.next();
                    continue;
                }
            }

            buffer.push(instr.clone());
        }

        self.content = buffer;
    }
}
