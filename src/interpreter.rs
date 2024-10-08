use std::{
    io::{self, Read},
    usize,
};

use crate::tokens::Token;

pub struct Interpreter {
    program: Vec<Token>,
    memory: Vec<u8>,
    dp: usize,
    ip: usize,
}

impl Interpreter {
    fn set_memory(&mut self, value: u8) {
        self.memory[self.dp] = value;
    }
    fn read_memory(&self) -> u8 {
        return self.memory[self.dp];
    }
    fn add_memory(&mut self, n: u32) {
        let (result, _) = self.memory[self.dp].overflowing_add(n as u8);
        self.memory[self.dp] = result;
    }
    fn sub_memory(&mut self, n: u32) {
        let (result, _) = self.memory[self.dp].overflowing_sub(n as u8);
        self.memory[self.dp] = result;
    }
    pub fn new(size: usize, program: Vec<Token>) -> Interpreter {
        return Interpreter {
            program,
            memory: vec![0; size],
            ip: 0,
            dp: 0,
        };
    }

    pub fn exec(&mut self) {
        while self.ip < self.program.len() {
            let maybe_token = self.program.get(self.ip);
            let token = match maybe_token {
                Some(v) => v,
                None => panic!("Critical error: attempted to access a token which does not exist"),
            };

            match token {
                Token::Input => {
                    let mut buff: [u8; 1] = [0];
                    while io::stdin().read_exact(&mut buff).is_err() {
                    }
                    self.set_memory(buff[0]);
                }
                Token::Output => print!("{}", self.read_memory() as char),
                Token::Inc(n) => self.add_memory(*n),
                Token::Dec(n) => self.sub_memory(*n),
                Token::LBrack(n) => {
                    if self.read_memory() == 0 {
                        self.ip = *n;
                        self.ip -= 1;
                    }
                }
                Token::RBrack(n) => {
                    if self.read_memory() != 0 {
                        self.ip = *n;
                        self.ip -= 1;
                    }
                }
                Token::IncDP(n) => self.dp += n,
                Token::DecDP(n) => self.dp -= n,
            }
            self.ip += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::Tokenizer;

    use super::*;

    #[test]
    fn test_add() {
        let tokens = Tokenizer::new("+++").tokenize();
        let mut interpreter = Interpreter::new(30, tokens);
        interpreter.exec();
        assert_eq!(interpreter.memory[0], 3);
    }

    #[test]
    fn test_sub() {
        let tokens = Tokenizer::new("---").tokenize();
        let mut interpreter = Interpreter::new(30, tokens);
        interpreter.exec();
        assert_eq!(interpreter.memory[0], 253);
    }

    #[test]
    fn test_move_dp_right() {
        let tokens = Tokenizer::new("+++>++>+").tokenize();
        let mut interpreter = Interpreter::new(30, tokens);
        interpreter.exec();
        assert_eq!(interpreter.memory[0], 3);
        assert_eq!(interpreter.memory[1], 2);
        assert_eq!(interpreter.memory[2], 1);
    }

    #[test]
    fn test_move_dp_left() {
        let tokens = Tokenizer::new("++>+<-").tokenize();
        let mut interpreter = Interpreter::new(30, tokens);
        interpreter.exec();
        assert_eq!(interpreter.memory[0], 1);
        assert_eq!(interpreter.memory[1], 1);
    }

    #[test]
    fn test_loop() {
        let tokens = Tokenizer::new(">>>-<<<+[>+]").tokenize();
        let mut interpreter = Interpreter::new(30, tokens);
        interpreter.exec();
        assert_eq!(interpreter.memory[0], 1);
        assert_eq!(interpreter.memory[1], 1);
        assert_eq!(interpreter.memory[2], 1);
        assert_eq!(interpreter.memory[3], 0);
    }

    #[test]
    fn test_jump_past() {
        let tokens = Tokenizer::new(">>>-<<<[>+]++").tokenize();
        let mut interpreter = Interpreter::new(30, tokens);
        interpreter.exec();
        assert_eq!(interpreter.memory[0], 2);
        assert_eq!(interpreter.memory[1], 0);
        assert_eq!(interpreter.memory[2], 0);
        assert_eq!(interpreter.memory[3], 255);
    }

    #[test]
    fn test_jump_past_2() {
        let tokens = Tokenizer::new("[]++").tokenize();
        let mut interpreter = Interpreter::new(30, tokens);
        interpreter.exec();
        assert_eq!(interpreter.memory[0], 2);
        assert_eq!(interpreter.memory[1], 0);
    }
}
