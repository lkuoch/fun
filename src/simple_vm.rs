/// Simple stack based virtual machine

type Word = usize;

#[derive(Debug)]
enum OpCodes {
    PUSH(Word),
    ADD,
    MINUS,
}

const PROGRAM: [OpCodes; 5] = [
    OpCodes::PUSH(3),
    OpCodes::PUSH(4),
    OpCodes::ADD,
    OpCodes::PUSH(5),
    OpCodes::MINUS,
];

fn virtual_machine(program: &[OpCodes; 5]) -> Option<Word> {
    let mut stack = Vec::<Word>::new();

    for op in program {
        match op {
            OpCodes::PUSH(value) => {
                stack.push(*value);
            }
            inner_op @ (OpCodes::ADD | OpCodes::MINUS) => {
                let right = stack.pop().unwrap_or_default();
                let left = stack.pop().unwrap_or_default();

                match inner_op {
                    OpCodes::ADD => {
                        stack.push(left + right);
                    }
                    OpCodes::MINUS => {
                        stack.push(left - right);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    stack.last().cloned()
}

fn main() {
    let result = virtual_machine(&PROGRAM);
    println!("result: {:?}", result);
}
