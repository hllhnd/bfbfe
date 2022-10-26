use bfbfe_ir::block::IRBlock;
use bfbfe_ir::instruction::IRInstruction;

pub fn compile_to_c(block: &IRBlock) -> String
{
    let mut code = String::new();
    let mut indent_level = 1_usize;

    _compile_to_c(block, &mut code, &mut indent_level);

    code
}

fn _compile_to_c(block: &IRBlock, code: &mut String, indent_level: &mut usize)
{
    macro_rules! indent {
        ($level:expr, $content:expr) => {
            format!("{}{}", "    ".repeat($level), $content)
        };
    }

    macro_rules! push_raw {
        ($content:expr) => {
            code.push_str($content)
        };
    }

    macro_rules! push {
        ($content:expr) => {
            push_raw!(indent!(*indent_level, $content).as_str())
        };
    }

    macro_rules! sign {
        ($value:expr) => {
            if $value.is_negative() {
                "-"
            } else {
                "+"
            }
        };
    }

    macro_rules! access_value {
        ($shift:expr) => {
            format!(
                "tape[ptr{}]",
                if $shift == 0 {
                    String::new()
                } else {
                    format!(" {} {}", sign!($shift), $shift.abs())
                }
            )
        };
    }

    for instr in block.iter().by_ref() {
        match instr {
            IRInstruction::BeginProgram => {
                push_raw!(
                    "#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

#define TAPE_SIZE 30000

int main(void)
{
    uint8_t tape[TAPE_SIZE] = { 0 };
    size_t ptr = 0;

"
                );
            }

            IRInstruction::EndProgram => {
                push_raw!(
                    "
    return 0;
}
"
                );
            }

            IRInstruction::TraverseBy {
                val,
            } => {
                push!(format!("ptr {}= {};\n", sign!(val), val.abs()));
            }

            IRInstruction::MutateValue {
                pos,
                val,
            } => {
                push!(format!("{} {}= {};\n", access_value!(*pos), sign!(val), val.abs()));
            }

            IRInstruction::SetTo {
                pos,
                val,
            } => {
                push!(format!("{} = {};\n", access_value!(*pos), val));
            }

            IRInstruction::OutputBytes {
                poslst,
            } => {
                push!(format!(
                    "printf(\"{}\"{});\n",
                    "%c".repeat(poslst.len()),
                    poslst
                        .iter()
                        .map(|&i| format!(", {}", access_value!(i)))
                        .collect::<String>()
                ));
            }

            IRInstruction::ReadBytes {
                poslst,
            } => {
                for pos in poslst {
                    push!(format!("{} = getchar();\n", access_value!(*pos)));
                }
            }

            IRInstruction::ConditionalBlock(block) => {
                push!("while (tape[ptr] != 0) {\n");
                *indent_level += 1;
                _compile_to_c(block, code, indent_level);
                *indent_level -= 1;
                push!("}\n");
            }

            // BFBFE IR is subject to expansion, so instead of potentially generating invalid code, fail
            _ => {
                panic!("Unrecognized instruction");
            }
        }
    }
}
