use std::{error::Error, fs};

#[derive(Debug, Clone)]
struct Computer {
    ra: usize,
    rb: usize,
    rc: usize,
    ip: usize,
    program: Vec<u8>,
    out: Vec<u8>,
}

impl Computer {
    fn new(source: &str) -> Self {
        let register_offset = "Register #: ".len();
        let mut split_info = source.split('\n');
        // Populate Registers with initial values.
        let ra = split_info.next().expect("Expect Register A value.").trim()[register_offset..]
            .parse()
            .expect("Failed to parse initial value for Register A.");
        let rb = split_info.next().expect("Expect Register B value.").trim()[register_offset..]
            .parse()
            .expect("Failed to parse initial value for Register B.");
        let rc = split_info.next().expect("Expect Register C value.").trim()[register_offset..]
            .parse()
            .expect("Failed to parse initial value for Register C.");
        // Consume empty line
        split_info.next();
        // Parses program instructions
        let mut program = Vec::new();
        let program_offset = "Program: ".len();
        split_info
            .next()
            .expect("Expect Program instructions.")
            .trim()[program_offset..]
            .split(',')
            .for_each(|opcode| program.push(opcode.parse().expect("Failed to parse opcode.")));

        Computer {
            ra,
            rb,
            rc,
            ip: 0,
            program,
            out: Vec::new(),
        }
    }

    fn run(&mut self) {
        while self.ip + 1 < self.program.len() {
            self.execute(self.program[self.ip], self.program[self.ip + 1]);
        }
    }

    fn execute(&mut self, opcode: u8, operand: u8) {
        match opcode {
            // adv
            0 => {
                self.ra = self.ra / 2usize.pow(self.combo(operand) as u32);
            }
            // bxl
            1 => {
                self.rb ^= operand as usize;
            }
            // bst
            2 => {
                self.rb = self.combo(operand) % 8;
            }
            // jnz
            3 => {
                if self.ra != 0 {
                    self.ip = operand as usize;
                    return;
                }
            }
            // bxc
            4 => {
                self.rb = self.rb ^ self.rc;
            }
            // out
            5 => {
                self.out.push((self.combo(operand) % 8) as u8);
            }
            // bdv
            6 => {
                self.rb = self.ra / 2usize.pow(self.combo(operand) as u32);
            }
            // cdv
            7 => {
                self.rc = self.ra / 2usize.pow(self.combo(operand) as u32);
            }
            _ => unreachable!("Valid opcodes are 0-7."),
        }
        self.ip += 2;
    }

    fn combo(&self, combo_operand: u8) -> usize {
        match combo_operand {
            0..=3 => combo_operand as usize,
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            _ => unreachable!("Valid combo operands are 0-6."),
        }
    }

    fn print_out(&self) {
        for idx in 0..self.out.len() {
            if idx != 0 {
                print!(",");
            }
            print!("{}", self.out[idx]);
        }
        print!("\n");
    }

    fn clear(&mut self) {
        self.ra = 0;
        self.rb = 0;
        self.rc = 0;
        self.ip = 0;
        self.out.clear();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::new(&fs::read_to_string("day-17/day17_input.txt")?);
    computer.run();
    computer.print_out();

    let mut ra_options = vec![0];
    let mut next_ra_set = Vec::new();
    for program_offset in (0..computer.program.len()).rev() {
        for &starting_ra in ra_options.iter() {
            // Program shifts register A left 3 bits on every iteration, so working backwards
            // testing each combination to find one that produces the target program.
            for test_bits in 0b000..=0b111 {
                computer.clear();
                computer.ra = (starting_ra << 3) ^ test_bits;
                computer.run();
                if computer.out == computer.program[program_offset..] {
                    next_ra_set.push((starting_ra << 3) ^ test_bits);
                }
            }
        }
        ra_options = next_ra_set.clone();
        next_ra_set.clear();
    }
    let min_valid_ra = ra_options.into_iter().min().unwrap();
    println!("Lowest initial register A value: {min_valid_ra}");

    Ok(())
}
