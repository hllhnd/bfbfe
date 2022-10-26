use crate::block::IRBlock;
use crate::instruction::IRInstruction;
pub trait ReorderingOptimizations
{
    fn reorder_instructions(&mut self);
}

impl ReorderingOptimizations for IRBlock
{
    fn reorder_instructions(&mut self)
    {
        fn cleanup(
            new_block: &mut Vec<IRInstruction>,
            set_moves: &mut Vec<(isize, isize)>,
            mutate_moves: &mut Vec<(isize, isize)>,
            ptr_shift: &mut isize,
        )
        {
            if !set_moves.is_empty() {
                let mut new_instrs = Vec::<IRInstruction>::new();

                set_moves.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

                for (pos, val) in &mut *set_moves {
                    new_instrs.push(IRInstruction::SetTo {
                        pos: *pos, val: *val
                    });
                }

                new_block.extend(new_instrs);

                *set_moves = Vec::new();
            }

            if !mutate_moves.is_empty() {
                let mut new_instrs = Vec::<IRInstruction>::new();

                mutate_moves.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

                for (pos, val) in &mut *mutate_moves {
                    new_instrs.push(IRInstruction::MutateValue {
                        pos: *pos, val: *val
                    });
                }

                new_block.extend(new_instrs);

                *mutate_moves = Vec::new();
            }

            if *ptr_shift != 0 {
                new_block.push(IRInstruction::TraverseBy {
                    val: *ptr_shift
                });

                *ptr_shift = 0;
            }
        }

        let mut new_block: Vec<IRInstruction> = Vec::new();

        let mut mutate_moves = Vec::<(isize, isize)>::new();
        let mut set_moves = Vec::<(isize, isize)>::new();
        let mut ptr_shift = 0_isize;

        for instr in &self.content {
            match instr {
                IRInstruction::TraverseBy {
                    val,
                } => {
                    ptr_shift += val;
                }

                IRInstruction::MutateValue {
                    pos,
                    val,
                } => {
                    mutate_moves.push((pos + ptr_shift, *val));
                }

                IRInstruction::SetTo {
                    pos,
                    val,
                } => {
                    set_moves.push((pos + ptr_shift, *val));
                }

                _ => {
                    cleanup(&mut new_block, &mut set_moves, &mut mutate_moves, &mut ptr_shift);

                    new_block.push(instr.clone());
                }
            }
        }

        cleanup(&mut new_block, &mut set_moves, &mut mutate_moves, &mut ptr_shift);

        self.content = new_block;
    }
}
