use std::io::{self, Write};

enum Command {
    Addition,
    Subtraction,
    PointerLeft,
    PointerRight,
    LoopStart,
    LoopEnd,
    Read,
    Write,
}

impl Command {
    fn from_char(c: &char) -> Result<Command, &'static str> {
        match c {
            '+' => Ok(Command::Addition),
            '-' => Ok(Command::Subtraction),
            '<' => Ok(Command::PointerLeft),
            '>' => Ok(Command::PointerRight),
            '[' => Ok(Command::LoopStart),
            ']' => Ok(Command::LoopEnd),
            ',' => Ok(Command::Read),
            '.' => Ok(Command::Write),
            _ => Err("invalid command"),
        }
    }
}

fn main() {
    let mut loop_stack: Vec<usize> = Vec::new();

    let program = "++++[>++++.<-.]";
    let mut program_pointer = 0;

    let mut memory: [u8; 10000] = [0; 10000];
    let mut pointer = 0;

    while program_pointer < program.len() {
        let c = program.chars().nth(program_pointer).unwrap();
        let command = Command::from_char(&c).unwrap();

        match command {
            Command::Addition => memory[pointer] = memory[pointer].wrapping_add(1),
            Command::Subtraction => memory[pointer] = memory[pointer].wrapping_sub(1),
            Command::PointerLeft => pointer = (pointer - 1) % memory.len(),
            Command::PointerRight => pointer = (pointer + 1) % memory.len(),
            Command::LoopStart => match memory[pointer] {
                0 => {
                    let mut depth = 1;

                    while depth > 0 {
                        program_pointer += 1;

                        let c = program.chars().nth(program_pointer).unwrap();

                        match Command::from_char(&c).unwrap() {
                            Command::LoopStart => depth += 1,
                            Command::LoopEnd => depth -= 1,
                            _ => (),
                        }
                    }
                }
                _ => loop_stack.push(program_pointer),
            },
            Command::LoopEnd => match memory[pointer] {
                0 => {
                    loop_stack.pop().expect("unexpected loop end character");
                }
                _ => {
                    program_pointer = loop_stack
                        .last()
                        .copied()
                        .expect("unexpected loop end character")
                }
            },
            Command::Read => {
                print!("enter an integer (0-255): ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("invalid input");

                memory[pointer] = match input.trim().parse::<u32>() {
                    Ok(num) => match num {
                        0..=255 => num as u8,
                        _ => {
                            println!("not in range");
                            continue;
                        }
                    },
                    _ => {
                        println!("not an integer");
                        continue;
                    }
                }
            }
            Command::Write => println!("{0}", memory[pointer]),
        }

        program_pointer += 1;
    }
}
