use std::{error::Error, fs, iter::Peekable, str::Chars};

fn main() -> Result<(), Box<dyn Error>> {
    let input_program = fs::read_to_string("day-3/day3_input.txt")?;
    let mut result = 0;
    let mut result_with_conditions = 0;

    let mut scanner = Scanner::new(&input_program);
    scanner.scan_tokens();

    let mut token_iter = scanner.tokens.into_iter();
    let mut is_enabled = true;
    while let Some(next_token) = token_iter.next() {
        match next_token {
            Token::Mul(first_num, second_num) => {
                result += first_num * second_num;
                if is_enabled {
                    result_with_conditions += first_num * second_num;
                }
            }
            Token::Do => is_enabled = true,
            Token::Dont => is_enabled = false,
        }
    }

    println!("Program result: {result}");
    println!("Program result with conditions: {result_with_conditions}");

    Ok(())
}

struct Scanner<'a> {
    program_source: &'a str,
    program_iter: Peekable<Chars<'a>>,
    current: usize,
    tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
    fn new(program_source: &'a str) -> Self {
        let program_iter = program_source.chars().peekable();
        Scanner {
            program_source,
            program_iter,
            tokens: Vec::new(),
            current: 0,
        }
    }

    fn scan_tokens(&mut self) {
        while self.current < self.program_source.len() {
            self.scan_token();
        }
    }

    fn scan_token(&mut self) {
        match self.advance() {
            'd' => self.conditional(),
            'm' => self.mul(),
            _ => (),
        }
    }

    fn conditional(&mut self) {
        // Checks for 'o'.
        match self.program_iter.peek() {
            Some(&'o') => {
                self.advance();
            }
            _ => {
                return;
            }
        }

        // Checks for end of do() condition.
        if self.program_iter.peek() == Some(&'(') {
            self.advance();
            match self.program_iter.peek() == Some(&')') {
                true => {
                    self.advance();
                    self.tokens.push(Token::Do);
                }
                false => (),
            }
            return;
        }

        // Continue check for don't() condition.
        // Checks for 'n'.
        match self.program_iter.peek() {
            Some(&'n') => {
                self.advance();
            }
            _ => {
                return;
            }
        }
        // Checks for '\''.
        match self.program_iter.peek() {
            Some(&'\'') => {
                self.advance();
            }
            _ => {
                return;
            }
        }
        // Checks for 't'.
        match self.program_iter.peek() {
            Some(&'t') => {
                self.advance();
            }
            _ => {
                return;
            }
        }

        // Checks for end of don't() condition.
        if self.program_iter.peek() == Some(&'(') {
            self.advance();
            match self.program_iter.peek() == Some(&')') {
                true => {
                    self.advance();
                    self.tokens.push(Token::Dont);
                }
                false => (),
            }
        }
    }

    fn mul(&mut self) {
        // Checks for 'u'.
        match self.program_iter.peek() {
            Some(&'u') => {
                self.advance();
            }
            _ => {
                return;
            }
        }
        // Checks for 'l'.
        match self.program_iter.peek() {
            Some(&'l') => {
                self.advance();
            }
            _ => {
                return;
            }
        }
        // Checks for '('.
        match self.program_iter.peek() {
            Some(&'(') => {
                self.advance();
            }
            _ => {
                return;
            }
        }
        // Checks for first number
        let first_number = match self.program_iter.peek() {
            Some(&next_char) => match next_char {
                '0' => {
                    self.advance();
                    0
                }
                '1'..='9' => self.number(),
                _ => {
                    return;
                }
            },
            _ => {
                return;
            }
        };
        // Checks for ','.
        match self.program_iter.peek() {
            Some(&',') => {
                self.advance();
            }
            _ => {
                return;
            }
        }
        // Checks for second number
        let second_number = match self.program_iter.peek() {
            Some(&next_char) => match next_char {
                '0' => {
                    self.advance();
                    0
                }
                '1'..='9' => self.number(),
                _ => {
                    return;
                }
            },
            _ => {
                return;
            }
        };
        // Checks for ')'.
        match self.program_iter.peek() {
            Some(&')') => {
                self.advance();
            }
            _ => {
                return;
            }
        }
        self.tokens.push(Token::Mul(first_number, second_number));
    }

    fn number(&mut self) -> i32 {
        let start = self.current;
        let mut digit_ctr = 0;
        while let Some(&next_char) = self.program_iter.peek() {
            // Numbers are limited to 1-3 digits.
            if digit_ctr == 3 {
                break;
            }
            match next_char {
                '0'..='9' => {
                    self.advance();
                    digit_ctr += 1;
                }
                _ => break,
            }
        }
        let number = self.program_source[start..self.current]
            .parse::<i32>()
            .expect(&format!(
                "Failed to parse lexeme '{}' to an i32.",
                &self.program_source[start..self.current]
            ));
        number
    }

    fn advance(&mut self) -> char {
        match self.program_iter.next() {
            Some(c) => {
                self.current += c.len_utf8();
                c
            }
            None => '\0',
        }
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Mul(i32, i32),
    Do,
    Dont,
}
