use std::usize;

use crate::tokens::Token;

pub struct Interpreter {
    program: Vec<Token>,
    memory: Vec<u8>,
    dp: usize,
    ip: usize,
}

impl Interpreter {
    fn read_memory(&self) -> u8 {
        return self.memory[self.dp];
    }
    fn add_memory(&mut self, n: u32) {
        self.memory[self.dp] += n as u8;
    }
    fn sub_memory(&mut self, n: u32) {
        self.memory[self.dp] -= n as u8;
    }
    fn branch_left(&mut self) {
        // if the memory is not zero, skip the bracket
        if self.read_memory() != 0 {
            self.ip += 1;
            return;
        }

        // go to matching closing bracket
        let mut depth = 1;
        while depth > 0 {
            let token = self.program.get(self.ip);
            match token {
                Some(v) => match v {
                    Token::LBrack => depth += 1,
                    Token::RBrack => depth -= 1,
                    _ => {}
                },
                None => panic!("No matching closing bracket found!"),
            }
            self.ip += 1;
        }
    }

    fn branch_right(&mut self) {
        // if the memory is not zero, skip the bracket
        if self.read_memory() != 0 {
            self.ip += 1;
            return;
        }

        // go to matching opening bracket
        let mut depth = 1;
        while depth > 0 {
            let token = self.program.get(self.ip);
            match token {
                Some(v) => match v {
                    Token::LBrack => depth -= 1,
                    Token::RBrack => depth += 1,
                    _ => {}
                },
                None => panic!("No matching closing bracket found!"),
            }
            self.ip -= 1;
        }
    }

    pub fn new(size: usize, program: Vec<Token>) -> Interpreter {
        return Interpreter {
            program,
            memory: vec![0; size],
            ip: 0,
            dp: 0,
        };
    }

    pub fn exec(&mut self, tokens: &[Token]) {
        while self.ip < tokens.len() {
            let maybe_token = tokens.get(self.ip);
            let token = match maybe_token {
                Some(v) => v,
                None => panic!("Critical error: attempted to access a token which does not exist"),
            };

            match token {
                Token::Input => todo!(),
                Token::Output => print!("{}", self.read_memory()),
                Token::Inc(n) => self.add_memory(*n),
                Token::Dec(n) => self.sub_memory(*n),
                Token::LBrack => self.branch_left(),
                Token::RBrack => self.branch_right(),
                Token::IncDP(n) => self.dp -= n,
                Token::DecDP(n) => self.dp -= n,
            }
        }
    }
}
