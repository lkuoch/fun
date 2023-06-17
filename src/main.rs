#[macro_use]
extern crate lazy_static;

use anyhow::{anyhow, Result};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u8)]
enum Op {
    Constant = 0,
}

type OpCode = u8;
type Instructions = Vec<OpCode>;

#[derive(Debug, Clone)]
struct Definition {
    name: Arc<&'static str>,

    // Number of bytes each operand takes
    operand_widths: Arc<[usize]>,
}

lazy_static! {
    static ref DEFINITIONS: HashMap<Op, Definition> = vec![(
        Op::Constant,
        Definition {
            name: Arc::new("OpConstant"),
            operand_widths: Arc::new([2]),
        },
    )]
    .into_iter()
    .collect();
}

fn lookup(op: &Op) -> Result<Definition> {
    DEFINITIONS
        .get(op)
        .cloned()
        .ok_or_else(|| anyhow!("Opcode {:?} was not defined", op))
}

fn make(op: Op, operands: Vec<isize>) -> Result<Instructions> {
    let def = lookup(&op)?;

    let mut instruction = Vec::with_capacity(1 + def.operand_widths.iter().sum::<usize>());
    instruction.push(op as OpCode);

    for (i, &operand) in operands.iter().enumerate() {
        let width = def.operand_widths[i];
        let bytes: [OpCode; 2] = (operand as u16).to_be_bytes();

        match width {
            2 => {
                instruction.extend_from_slice(&bytes);
            }
            _ => unimplemented!("TODO LATER"),
        }
    }

    Ok(instruction)
}

fn main() {
    println!("{:?}", *DEFINITIONS);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make() {
        let test_cases = vec![(
            Op::Constant,
            vec![65534],
            vec![Op::Constant as u8, 255, 254],
        )];

        for (op, operands, expected) in test_cases {
            let instructions = make(op, operands).unwrap();

            assert_eq!(
                instructions.len(),
                expected.len(),
                "instruction has wrong len. instructions={instructions:?}, expected={expected:?}"
            );

            for (idx, &b) in expected.iter().enumerate() {
                let instruction = instructions[idx];

                assert_eq!(
                    instruction, b,
                    "wrong byte at pos {idx}. instruction={instruction:?}, b={b:?}"
                );
            }
        }
    }
}
