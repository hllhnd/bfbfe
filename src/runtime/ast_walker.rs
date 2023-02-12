use std::io::Read;

use crate::ast::Element;
use crate::ast::Node;

pub fn interpret_ast(node: &Node)
{
    fn run_node(node: &Node, tape: &mut [u8; 30_000], ptr: &mut usize)
    {
        for element in &node.content {
            match element {
                Element::MovPtr {
                    by,
                } => *ptr = ptr.wrapping_add_signed(*by),
                Element::MutVal {
                    at,
                    by,
                } => {
                    tape[ptr.wrapping_add_signed(*at)] =
                        tape[ptr.wrapping_add_signed(*at)].wrapping_add_signed(*by as i8);
                }

                Element::Read {
                    to,
                } => tape[ptr.wrapping_add_signed(*to)] = std::io::stdin().bytes().next().unwrap().unwrap(),

                Element::Push {
                    from,
                } => print!("{}", tape[ptr.wrapping_add_signed(*from)] as char),

                Element::CondBlck {
                    node,
                } => {
                    while tape[*ptr] != 0 {
                        run_node(node, tape, ptr);
                    }
                }
            }
        }
    }

    let mut tape = [0; 30_000];
    let mut ptr = 0;

    run_node(node, &mut tape, &mut ptr);
}
