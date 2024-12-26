use std::{collections::HashMap, error::Error, fs, iter::Peekable, str::Chars};

fn main() -> Result<(), Box<dyn Error>> {
    let device_config = fs::read_to_string("day-24/day24_input.txt")?;

    let mut wires: HashMap<&str, bool> = HashMap::new();
    let mut logic_gates: Vec<LogicGate<'_>> = vec![];

    let mut scanner = ConfigScanner::new(&device_config);
    scanner.scan_device_config();

    parse_config(&scanner.tokens, &mut wires, &mut logic_gates);
    let input_nums_bit_len = wires.len() / 2;

    // Get output for existing circuit.
    run_device(&mut wires, &logic_gates);
    let z_wire_output = get_value_across_wire('z', &wires);
    println!("z wire output with current config: {z_wire_output}");

    // Find which wires to swap to fix ripple carry adder circuit.
    let mut swapped_wires = repair_ripple_carry_adder(input_nums_bit_len, &mut logic_gates);
    swapped_wires.sort();
    println!("{}", swapped_wires.join(","));

    Ok(())
}

fn repair_ripple_carry_adder<'source, 'gates>(
    input_bit_len: usize,
    logic_gates: &'gates mut Vec<LogicGate<'source>>,
) -> Vec<&'source str> {
    let mut swapped_wires = vec![];

    let mut input_carry_wire = "";
    for bit_num in 0..input_bit_len {
        let mut bit_num_str = bit_num.to_string();
        if bit_num <= 9 {
            bit_num_str = format!("0{bit_num_str}");
        }
        let x_input_wire = format!("x{bit_num_str}");
        let z_output_wire = format!("z{bit_num_str}");

        // Get idx of of all logic gates involved in nth bit.
        let mut x_y_xor_gate_idx = logic_gates.len();
        let mut x_y_and_gate_idx = logic_gates.len();
        let mut carry_in_xor_gate_idx = logic_gates.len();
        let mut carry_in_and_gate_idx = logic_gates.len();
        for idx in 0..logic_gates.len() {
            let gate = &logic_gates[idx];
            // Check for XOR gate
            match gate.operator {
                TokenType::Xor => {
                    if gate.left == x_input_wire || gate.right == x_input_wire {
                        x_y_xor_gate_idx = idx;
                    } else if gate.left == input_carry_wire || gate.right == input_carry_wire {
                        carry_in_xor_gate_idx = idx;
                    }
                }
                TokenType::And => {
                    if gate.left == x_input_wire || gate.right == x_input_wire {
                        x_y_and_gate_idx = idx;
                    } else if gate.left == input_carry_wire || gate.right == input_carry_wire {
                        carry_in_and_gate_idx = idx;
                    }
                }
                _ => (),
            }
        }
        if bit_num == 0 {
            // Swap if input XOR gate doesn't output to first z00 bit.
            if logic_gates[x_y_xor_gate_idx].output != z_output_wire {
                swap_outputs(
                    x_y_xor_gate_idx,
                    x_y_and_gate_idx,
                    logic_gates,
                    &mut swapped_wires,
                );
            }
            input_carry_wire = logic_gates[x_y_and_gate_idx].output;
        } else {
            let mut carry_out_guess_from_carry_in = logic_gates.len();
            let mut carry_out_guess_from_x_y = logic_gates.len();
            let carry_out_input_from_carry_in = logic_gates[carry_in_and_gate_idx].output;
            let carry_out_input_from_x_y = logic_gates[x_y_and_gate_idx].output;
            for idx in 0..logic_gates.len() {
                let gate = &logic_gates[idx];
                if gate.operator == TokenType::Or {
                    if gate.left == carry_out_input_from_carry_in
                        || gate.right == carry_out_input_from_carry_in
                    {
                        carry_out_guess_from_carry_in = idx;
                    }
                    if gate.left == carry_out_input_from_x_y
                        || gate.right == carry_out_input_from_x_y
                    {
                        carry_out_guess_from_x_y = idx;
                    }
                }
            }
            // Verify carry_in_xor_gate output (output z wire).
            if logic_gates[carry_in_xor_gate_idx].output != z_output_wire {
                let mut swap_idx = logic_gates.len();
                if logic_gates[x_y_xor_gate_idx].output == z_output_wire {
                    swap_idx = x_y_xor_gate_idx;
                } else if logic_gates[x_y_and_gate_idx].output == z_output_wire {
                    swap_idx = x_y_and_gate_idx;
                    carry_out_guess_from_x_y = carry_out_guess_from_carry_in;
                } else if logic_gates[carry_in_and_gate_idx].output == z_output_wire {
                    swap_idx = carry_in_and_gate_idx;
                    carry_out_guess_from_carry_in = carry_out_guess_from_x_y;
                } else if carry_out_guess_from_carry_in < logic_gates.len()
                    && logic_gates[carry_out_guess_from_carry_in].output == z_output_wire
                {
                    swap_idx = carry_out_guess_from_carry_in;
                } else if carry_out_guess_from_x_y < logic_gates.len()
                    && logic_gates[carry_out_guess_from_x_y].output == z_output_wire
                {
                    swap_idx = carry_out_guess_from_x_y;
                }
                swap_outputs(
                    carry_in_xor_gate_idx,
                    swap_idx,
                    logic_gates,
                    &mut swapped_wires,
                );
            }
            // Verify x_y_xor_gate output.
            let mut expected_x_y_xor_output = logic_gates[carry_in_xor_gate_idx].left;
            if expected_x_y_xor_output == input_carry_wire {
                expected_x_y_xor_output = logic_gates[carry_in_xor_gate_idx].right;
            }
            if logic_gates[x_y_xor_gate_idx].output != expected_x_y_xor_output {
                let mut swap_idx = logic_gates.len();
                if logic_gates[x_y_and_gate_idx].output == expected_x_y_xor_output {
                    swap_idx = x_y_and_gate_idx;
                } else if logic_gates[carry_in_and_gate_idx].output == expected_x_y_xor_output {
                    swap_idx = carry_in_and_gate_idx;
                    carry_out_guess_from_carry_in = carry_out_guess_from_x_y;
                } else if carry_out_guess_from_carry_in < logic_gates.len()
                    && logic_gates[carry_out_guess_from_carry_in].output == expected_x_y_xor_output
                {
                    swap_idx = carry_out_guess_from_carry_in;
                } else if carry_out_guess_from_x_y < logic_gates.len()
                    && logic_gates[carry_out_guess_from_x_y].output == expected_x_y_xor_output
                {
                    swap_idx = carry_out_guess_from_x_y;
                }
                swap_outputs(x_y_xor_gate_idx, swap_idx, logic_gates, &mut swapped_wires);
            }

            input_carry_wire = logic_gates[carry_out_guess_from_carry_in].output;
        }
    }

    swapped_wires
}

fn swap_outputs<'source>(
    gate_1_idx: usize,
    gate_2_idx: usize,
    logic_gates: &mut Vec<LogicGate<'source>>,
    swapped_wires: &mut Vec<&'source str>,
) {
    let tmp = logic_gates[gate_1_idx].output;
    logic_gates[gate_1_idx].output = logic_gates[gate_2_idx].output;
    logic_gates[gate_2_idx].output = tmp;
    swapped_wires.push(logic_gates[gate_2_idx].output);
    swapped_wires.push(logic_gates[gate_1_idx].output);
}

fn get_value_across_wire<'source, 'state>(
    target_wire: char,
    wires: &'state HashMap<&'source str, bool>,
) -> usize {
    let mut target_wires = vec![];
    for wire in wires.keys() {
        if wire.starts_with(target_wire) {
            target_wires.push(*wire);
        }
    }
    target_wires.sort();
    let mut target_wire_value = 0;
    for wire in target_wires.into_iter().rev() {
        match wires.get(wire).unwrap() {
            true => {
                target_wire_value = (target_wire_value << 1) ^ 1;
            }
            false => {
                target_wire_value = target_wire_value << 1;
            }
        }
    }

    target_wire_value
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
