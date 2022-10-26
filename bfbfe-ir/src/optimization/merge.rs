use crate::block::IRBlock;
use crate::instruction::IRInstruction;

pub trait MergeInstructions
{
    fn merge_instructions(&mut self);
}

impl MergeInstructions for IRBlock
{
    fn merge_instructions(&mut self)
    {
        let mut buffer: Vec<IRInstruction> = Vec::new();

        let it = self.content.iter();
        let mut pk = it.peekable();

        'a: while let Some(instr) = pk.next() {
            match &instr {
                IRInstruction::TraverseBy {
                    val: mut lval,
                } => {
                    while let Some(IRInstruction::TraverseBy {
                        val: rval,
                    }) = pk.peek()
                    {
                        lval += rval;
                        pk.next();
                    }

                    buffer.push(IRInstruction::TraverseBy {
                        val: lval
                    });

                    continue 'a;
                }

                IRInstruction::MutateValue {
                    pos: lpos,
                    val: mut lval,
                } => {
                    'b: {
                        while let Some(IRInstruction::MutateValue {
                            pos: rpos,
                            val: rval,
                        }) = pk.peek()
                        {
                            if lpos != rpos {
                                break 'b;
                            }
                            lval += rval;
                            pk.next();
                        }

                        break 'b;
                    }

                    buffer.push(IRInstruction::MutateValue {
                        pos: *lpos, val: lval
                    });

                    continue 'a;
                }

                // IRInstruction::OutputBytes {
                //     poslst: lposlst,
                // } => {
                //     let mut lposlst = lposlst.clone();
                //     while let Some(IRInstruction::OutputBytes {
                //         poslst: rposlst,
                //     }) = pk.peek()
                //     {
                //         lposlst.extend(rposlst);
                //     }

                //     buffer.push(IRInstruction::OutputBytes {
                //         poslst: lposlst
                //     });

                //     continue 'a;
                // }

                _ => {}
            }

            buffer.push(instr.clone());
        }

        self.content = buffer;
    }
}
