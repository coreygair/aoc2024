type Input = (Registers, Vec<u64>);

pub fn parse(input: &str) -> Input {
    let mut ls = input.lines();

    let a = ls
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let b = ls
        .next()
        .unwrap()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let c = ls
        .next()
        .unwrap()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    ls.next();
    let program = ls
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .chars()
        .filter_map(|c| {
            if c == ',' {
                None
            } else {
                Some(c.to_digit(10).unwrap() as u64)
            }
        })
        .collect();

    (Registers { a, b, c }, program)
}

pub fn part1((registers, program): &Input) -> String {
    let computer = Computer {
        registers: registers.clone(),
        program,
    };

    let output = computer.run();

    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn part2((_, program): &Input) -> u64 {
    // By looking at the instructions in a run of part 1,
    // see that the program is a `while A != 0` loop
    // with exactly one instruction that modifies A, an `adv 3` (a >>= 3).
    //
    // For last output, try A in 0..8 to find what generates that last output.
    // Then, for last 2 outputs, try A in (A_{-1} << 3) + 0..8 to find what generates last 2 outputs.
    // So on...

    let mut steps = vec![0; program.len() + 1];

    let mut i = program.len() - 1;
    loop {
        let target = &program[i..];

        let j_start = if steps[i] == 0 {
            steps[i + 1] << 3
        } else {
            steps[i]
        };
        let j_end = (steps[i + 1] << 3) + 8;

        let mut found = false;
        for a in j_start..j_end {
            let output = Computer {
                registers: Registers { a, b: 0, c: 0 },
                program,
            }
            .run();

            if output == target {
                steps[i] = a;
                found = true;
                break;
            }
        }

        if !found {
            steps[i] = 0;
            i += 1;
            steps[i] += 1;
            continue;
        }

        if i == 0 {
            break;
        }

        i -= 1;
    }

    steps[0]
}

pub struct Computer<'a> {
    program: &'a Vec<u64>,
    registers: Registers,
}

impl<'a> Computer<'a> {
    fn run(mut self) -> Vec<u64> {
        let mut instruction_ptr = 0;
        let mut output = Vec::new();

        while instruction_ptr < self.program.len() {
            let instruction: Instruction = self.program[instruction_ptr].into();
            let operand = self.program[instruction_ptr + 1];

            match instruction.execute(&mut self.registers, operand) {
                InstructionResult::Unit => {}
                InstructionResult::Output(x) => output.push(x),
                InstructionResult::Jump(to) => {
                    instruction_ptr = to as usize;
                    continue;
                }
            }

            instruction_ptr += 2;
        }

        output
    }
}

#[derive(Clone, Debug)]
pub struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

impl Registers {
    fn combo_op(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!("unrecognized combo operand"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u64> for Instruction {
    fn from(value: u64) -> Self {
        match value {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => unreachable!("unrecognized instr"),
        }
    }
}

impl Instruction {
    fn execute(self, registers: &mut Registers, operand: u64) -> InstructionResult {
        match self {
            Instruction::Adv => registers.a >>= registers.combo_op(operand),
            Instruction::Bdv => registers.b = registers.a >> registers.combo_op(operand),
            Instruction::Cdv => registers.c = registers.a >> registers.combo_op(operand),

            Instruction::Bxl => registers.b ^= operand,
            Instruction::Bst => registers.b = registers.combo_op(operand) % 8,
            Instruction::Bxc => registers.b ^= registers.c,

            Instruction::Jnz => {
                if registers.a != 0 {
                    return InstructionResult::Jump(operand);
                }
            }
            Instruction::Out => return InstructionResult::Output(registers.combo_op(operand) % 8),
        }

        InstructionResult::Unit
    }
}

enum InstructionResult {
    Unit,
    Output(u64),
    Jump(u64),
}
