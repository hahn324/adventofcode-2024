use std::{collections::HashMap, error::Error, fs, iter::Peekable, str::Chars};

fn main() -> Result<(), Box<dyn Error>> {
    let device_config = fs::read_to_string("day-24/day24_input.txt")?;

    let mut wires: HashMap<&str, bool> = HashMap::new();
    let mut logic_gates: Vec<LogicGate<'_>> = vec![];

    let mut scanner = ConfigScanner::new(&device_config);
    scanner.scan_device_config();

    parse_config(&scanner.tokens, &mut wires, &mut logic_gates);

    run_device(&mut wires, &logic_gates);

    let mut z_wires = vec![];
    for wire in wires.keys() {
        if wire.starts_with('z') {
            z_wires.push(*wire);
        }
    }
    z_wires.sort();
    let mut z_wire_output: usize = 0;
    for wire in z_wires.into_iter().rev() {
        match wires.get(wire).unwrap() {
            true => {
                z_wire_output = (z_wire_output << 1) ^ 1;
            }
            false => {
                z_wire_output = z_wire_output << 1;
            }
        }
    }

    println!("z wire output: {z_wire_output}");

    Ok(())
}

fn run_device<'source, 'state>(
    wires: &'state mut HashMap<&'source str, bool>,
    logic_gates: &'state Vec<LogicGate<'source>>,
) {
    let mut gates_evaluated = 0;
    let number_of_gates = logic_gates.len();
    while gates_evaluated != number_of_gates {
        assert!(gates_evaluated <= number_of_gates);
        for gate in logic_gates.iter() {
            if !wires.contains_key(gate.output) {
                if wires.contains_key(gate.left) && wires.contains_key(gate.right) {
                    let left = *wires.get(gate.left).unwrap();
                    let right = *wires.get(gate.right).unwrap();
                    let value = match gate.operator {
                        TokenType::And => left && right,
                        TokenType::Or => left || right,
                        TokenType::Xor => left ^ right,
                        _ => unreachable!(),
                    };
                    wires.insert(gate.output, value);
                    gates_evaluated += 1;
                }
            }
        }
    }
}

fn parse_config<'source, 'state>(
    tokens: &'source Vec<Token>,
    wires: &'state mut HashMap<&'source str, bool>,
    logic_gates: &'state mut Vec<LogicGate<'source>>,
) {
    let mut token_iter = tokens.iter().peekable();
    while let Some(token) = token_iter.next() {
        let left = token;
        if let Some(next_token) = token_iter.peek() {
            match next_token.token_type {
                TokenType::Set => {
                    // Consume Set token
                    token_iter.next();
                    // Store wire with initial value.
                    let value = match token_iter
                        .next()
                        .expect("Expect to find Value token after Set.")
                        .lexeme
                    {
                        "1" => true,
                        "0" => false,
                        _ => unreachable!("Initial values will only be '1' or '0'"),
                    };
                    wires.insert(left.lexeme, value);
                }
                _ => {
                    let operator = token_iter.next().expect("Expect logical operator token");
                    let right = token_iter.next().expect("Expect right wire of logic gate");
                    // Consume Assign token
                    token_iter.next();
                    let output_wire = token_iter.next().expect("Expect output wire of logic gate");
                    let logic_gate = LogicGate {
                        left: left.lexeme,
                        right: right.lexeme,
                        operator: operator.token_type,
                        output: output_wire.lexeme,
                    };
                    logic_gates.push(logic_gate);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenType {
    And,
    Or,
    Xor,
    Set,
    Output,
    Identifier,
    Value,
}

#[derive(Debug, Clone)]
struct Token<'source> {
    token_type: TokenType,
    lexeme: &'source str,
}

#[derive(Debug, Clone)]
struct LogicGate<'source> {
    left: &'source str,
    right: &'source str,
    operator: TokenType,
    output: &'source str,
}

#[derive(Debug, Clone)]
struct ConfigScanner<'source> {
    tokens: Vec<Token<'source>>,
    start: usize,
    current: usize,
    source: &'source str,
    source_iter: Peekable<Chars<'source>>,
}
impl<'source> ConfigScanner<'source> {
    fn new(device_config: &'source str) -> Self {
        ConfigScanner {
            tokens: vec![],
            start: 0,
            current: 0,
            source: device_config,
            source_iter: device_config.chars().peekable(),
        }
    }

    fn scan_device_config(&mut self) {
        while self.current < self.source.len() {
            self.start = self.current;
            self.scan_token();
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            ':' => self.add_token(TokenType::Set),
            '-' => {
                // Consume the '>' char in ->.
                self.advance();
                self.add_token(TokenType::Output);
            }
            'A' => {
                // Consume the 'N' and 'D' chars in AND.
                self.advance();
                self.advance();
                self.add_token(TokenType::And);
            }
            'O' => {
                // Consume the 'R' char in OR.
                self.advance();
                self.add_token(TokenType::Or);
            }
            'X' => {
                // Consume the 'O' and 'R' chars in XOR.
                self.advance();
                self.advance();
                self.add_token(TokenType::Xor);
            }
            '0' | '1' => self.add_token(TokenType::Value),
            ' ' | '\n' => (),
            _ => {
                while let Some(&c) = self.source_iter.peek() {
                    match c {
                        ' ' | '\n' | ':' => break,
                        _ => self.advance(),
                    };
                }
                self.add_token(TokenType::Identifier);
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            lexeme: &self.source[self.start..self.current],
        });
    }

    fn advance(&mut self) -> char {
        match self.source_iter.next() {
            Some(c) => {
                self.current += c.len_utf8();
                c
            }
            None => '\0',
        }
    }
}
