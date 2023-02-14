use std::io::stdin;
use std::io::stdout;
use std::io::Read;
use std::io::Stdin;
use std::io::Stdout;
use std::io::Write;

use crate::ast::Element;
use crate::ast::Node;

// Would not recommend changing this as the instruction set is meant to use
// 16-bit integers for indexing.
const TAPE_SIZE: usize = 30_000;

/// Represents an execution context for an AST-walking interpreter.
///
/// The program's state is contained within and is public, allowing for
/// inspecting the program's tape and pointer upon completion.
///
/// Preparing a basic context for interpreting can be done by constructing it
/// from its [`Default`], replacing the `node` with your own.
/// ```
/// let context = ASTInterpreterContext {
///     node: my_brainfuck_program,
///     ..Default::default()
/// };
/// ```
///
/// Note that the [`Default`] implementation uses stdin and stdio for I/O. If
/// you want to use other sources try something like this (they have to
/// implement [`Read`] and [`Write`], though):
/// ```
/// let context = ASTInterpreterContext {
///     node: my_brainfuck_program,
///     input: minion_memes(),
///     output: facebook_moms(),
///     ..Default::default()
/// };
/// ```
pub struct ASTInterpreterContext<R: Read, W: Write>
{
    pub main_node:    Node,
    pub tape:         [u8; TAPE_SIZE],
    pub data_pointer: usize,
    pub input:        R,
    pub output:       W,
}

impl Default for ASTInterpreterContext<Stdin, Stdout>
{
    #[inline]
    fn default() -> Self
    {
        Self {
            main_node:    Node::default(),
            tape:         [u8::default(); TAPE_SIZE],
            data_pointer: usize::default(),
            input:        stdin(),
            output:       stdout(),
        }
    }
}

impl<R: Read, W: Write> ASTInterpreterContext<R, W>
{
    /// Execute the parent node in the current context.
    pub fn run(&mut self)
    {
        fn inner<R: Read, W: Write>(context: &mut ASTInterpreterContext<R, W>, node: &Node)
        {
            for element in &node.content {
                match element {
                    Element::PointerAdd(arg) => {
                        context.data_pointer = context.data_pointer.wrapping_add(*arg as usize);
                    }

                    Element::PointerSub(arg) => {
                        context.data_pointer = context.data_pointer.wrapping_sub(*arg as usize);
                    }

                    Element::ValueSet(arg) => {
                        context.tape[context.data_pointer] = *arg;
                    }

                    Element::ValueAdd(arg) => {
                        context.tape[context.data_pointer] = context.tape[context.data_pointer].wrapping_add(*arg);
                    }

                    Element::ValueSub(arg) => {
                        context.tape[context.data_pointer] = context.tape[context.data_pointer].wrapping_sub(*arg);
                    }

                    Element::Push => {
                        write!(context.output, "{}", context.tape[context.data_pointer] as char).unwrap();
                    }

                    Element::Pull => {
                        context.tape[context.data_pointer] = context.input.by_ref().bytes().next().unwrap().unwrap();
                    }

                    Element::Conditional(arg) => {
                        while context.tape[context.data_pointer] != 0 {
                            inner(context, arg);
                        }
                    }
                }
            }
        }

        // hee hee hoo borrow checker
        let main_node_clone = self.main_node.clone();

        inner(self, &main_node_clone);
    }
}
