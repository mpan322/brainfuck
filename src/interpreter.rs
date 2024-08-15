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
        let (result, _) = self.memory[self.dp].overflowing_add(n as u8);
        self.memory[self.dp] = result;
    }
    fn sub_memory(&mut self, n: u32) {
        let (result, _) = self.memory[self.dp].overflowing_sub(n as u8);
        self.memory[self.dp] = result;
    }
    fn branch_left(&mut self) {
        // if the memory is not zero, skip the bracket
        if self.read_memory() != 0 {
            return;
        }

        // go to matching closing bracket
        let mut depth = 1;
        self.ip += 1;
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
        self.ip -= 1;
    }

    fn branch_right(&mut self) {
        // if the memory is not zero, skip the bracket
        if self.read_memory() == 0 {
            return;
        }

        // go to matching opening bracket
        let mut depth = 1;
        self.ip -= 1;
        while depth > 0 {
            let token = self.program.get(self.ip);
            match token {
                Some(v) => match v {
                    Token::LBrack => depth -= 1,
                    Token::RBrack => depth += 1,
                    _ => {}
                },
                None => panic!("No matching opening bracket found!"),
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

    pub fn exec(&mut self) {
        while self.ip < self.program.len() {
            let maybe_token = self.program.get(self.ip);
            let token = match maybe_token {
                Some(v) => v,
                None => panic!("Critical error: attempted to access a token which does not exist"),
            };
            // println!("{:?} {:?} -> {:?}", self.ip, token, self.dp);

            match token {
                Token::Input => todo!(),
                Token::Output => print!("{}", self.read_memory()),
                Token::Inc(n) => self.add_memory(*n),
                Token::Dec(n) => self.sub_memory(*n),
                Token::LBrack => self.branch_left(),
                Token::RBrack => self.branch_right(),
                Token::IncDP(n) => self.dp += n,
                Token::DecDP(n) => self.dp -= n,
            }
            self.ip += 1;
        }
        // println!("{:?}", self.memory);
        // println!("{:?}", self.ip);
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::Tokenizer;

    use super::*;

    #[test]
    fn test_add() {
        let tokenizer = Tokenizer::new("+++");
        let mut interpreter = Interpreter::new(30, tokenizer.collect());
        interpreter.exec();
        assert_eq!(interpreter.memory[0], 3);
    }

    #[test]
    fn test_sub() {
        let tokenizer = Tokenizer::new("---");
        let mut interpreter = Interpreter::new(30, tokenizer.collect());
        interpreter.exec();
        assert_eq!(interpreter.memory[0], 253);
    }

    #[test]
    fn test_move_dp_right() {
        let tokenizer = Tokenizer::new("+++>++>+");
        let mut interpreter = Interpreter::new(30, tokenizer.collect());
        interpreter.exec();
        assert_eq!(interpreter.memory[0], 3);
        assert_eq!(interpreter.memory[1], 2);
        assert_eq!(interpreter.memory[2], 1);
    }

    #[test]
    fn test_move_dp_left() {
        let tokenizer = Tokenizer::new("++>+<-");
        let mut interpreter = Interpreter::new(30, tokenizer.collect());
        interpreter.exec();
        assert_eq!(interpreter.memory[0], 1);
        assert_eq!(interpreter.memory[1], 1);
    }

    #[test]
    fn test_loop() {
        let tokenizer = Tokenizer::new(">>>-<<<+[>+]");
        let coll: Vec<Token> = tokenizer.collect();
        let mut interpreter = Interpreter::new(30, coll);
        interpreter.exec();
        assert_eq!(interpreter.memory[0], 1);
        assert_eq!(interpreter.memory[1], 1);
        assert_eq!(interpreter.memory[2], 1);
        assert_eq!(interpreter.memory[3], 0);
    }

    #[test]
    fn test_jump_past() {
        let tokenizer = Tokenizer::new(">>>-<<<[>+]++");
        let coll: Vec<Token> = tokenizer.collect();
        let mut interpreter = Interpreter::new(30, coll);
        interpreter.exec();
        assert_eq!(interpreter.memory[0], 2);
        assert_eq!(interpreter.memory[1], 0);
        assert_eq!(interpreter.memory[2], 0);
        assert_eq!(interpreter.memory[3], 255);
    }

    #[test]
    fn test_jump_past_2() {
        let tokenizer = Tokenizer::new("[]++");
        let coll: Vec<Token> = tokenizer.collect();
        let mut interpreter = Interpreter::new(30, coll);
        interpreter.exec();
        assert_eq!(interpreter.memory[0], 2);
        assert_eq!(interpreter.memory[1], 0);
    }
}
