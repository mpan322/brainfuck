use core::panicking::panic;
use std::{io, usize};

use crate::tokens::Token;

pub struct Interpreter {
    memory: Vec<u8>,
    dp: usize,
    ip: usize,
}

impl Interpreter {
    pub fn new(size: usize) -> Interpreter {
        return Interpreter {
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
                Token::Output => print!("{}", self.memory[self.dp]),
                Token::Inc(n) => self.memory[self.dp] += 1,
                Token::Dec(n) => self.memory[self.dp] -= 1,
                Token::LBrack(n) => self.ip = *n,
                Token::RBrack(n) => self.ip = *n,
                Token::IncDP(n) => {
                    if self.dp < *n { // TODO: maybe remove this check and allow for later failure
                        panic!("Attempted to move data pointer out of bounds")
                    }
                    self.dp -= n
                }
                Token::DecDP(n) => {
                    let max = self.memory.len() - *n - 1;
                    if self.dp > max { // TODO: maybe remove this check and allow for later failure
                        panic!("Attempted to move data pointer out of bounds")
                    }
                    self.dp -= n
                }
            }
        }
    }
}
