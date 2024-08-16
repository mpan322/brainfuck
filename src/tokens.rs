use core::panic;
use std::usize;

#[derive(Debug, PartialEq)]
pub enum Token {
    Inc(u32),
    Dec(u32),
    IncDP(usize),
    DecDP(usize),
    LBrack(usize),
    RBrack(usize),
    Output,
    Input,
}

pub struct Tokenizer {
    input: Vec<u8>,
    idx: usize,
    curr: u8,
    next: u8,
    done: bool,
}

fn is_repeatable(char: u8) -> bool {
    return match char {
        b'+' | b'-' | b'>' | b'<' => true,
        _ => false,
    };
}
fn is_valid_char(char: u8) -> bool {
    return match char {
        b'[' | b']' | b',' | b'.' | b'+' | b'-' | b'>' | b'<' => true,
        _ => false,
    };
}

fn string_to_vec(s: &str) -> Vec<u8> {
    let mut v = Vec::new();
    s.chars().for_each(|c| v.push(c as u8));
    return v;
}

impl Tokenizer {
    fn add_jumps(tokens: &mut Vec<Token>) -> () {
        let mut stack = Vec::new();
        for (i, token) in tokens.iter_mut().enumerate() {
            match *token {
                Token::LBrack(_) => stack.push((token, i)),
                Token::RBrack(_) => {
                    let paired = stack.pop();
                    let (corr, idx) = match paired {
                        Some(v) => v,
                        None => panic!("Unblanced brackets - too many closing brackets"),
                    };
                    *corr = Token::LBrack(i + 1);
                    *token = Token::RBrack(idx);
                }
                _ => (),
            }
        }

        if stack.len() > 0 {
            panic!("Unbalanced brackets - too many opening brackets");
        }
    }

    pub fn new(input: &str) -> Tokenizer {
        let input = string_to_vec(input);
        let next = input.get(0);
        let next = match next {
            None => 0,
            Some(v) => *v,
        };
        return Tokenizer {
            input,
            idx: 0,
            curr: 0,
            next,
            done: false,
        };
    }

    fn is_done(&self) -> bool {
        return self.done;
    }

    fn next_char(&mut self) -> bool {
        loop {
            let next_char = self.input.get(self.idx + 1);
            match next_char {
                Some(v) => {
                    // skip over invalid characters
                    if !is_valid_char(*v) {
                        self.idx += 1;
                        continue;
                    }
                    self.curr = self.next;
                    self.next = *v;
                    self.idx += 1;
                }
                None => {
                    self.curr = self.next;
                    self.next = 0;
                    if self.curr == 0 {
                        self.done = true
                    }
                }
            }
            // once a valid character is found, or the end of the input is reached, break out of the loop
            break;
        }
        return self.done;
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = self.collect();
        Tokenizer::add_jumps(&mut tokens);
        return tokens;
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // get the next character
        self.next_char();
        if self.is_done() {
            return None;
        }
        let char = self.curr;
        println!(
            "curr: {:?}, next: {:?}",
            self.curr as char, self.next as char
        );

        // condense sequences of characters which are repeatable into 1 token
        if is_repeatable(char) {
            let mut count: u32 = 1;
            while self.curr == self.next && !self.is_done() {
                self.next_char();
                count += 1;
            }
            let token = match char {
                b'+' => Token::Inc(count as u32),
                b'-' => Token::Dec(count as u32),
                b'<' => Token::DecDP(count as usize),
                b'>' => Token::IncDP(count as usize),
                _ => panic!("Unexpected character {} when tokenizing", char),
            };
            return Some(token);
        }

        // handle other tokens
        let token = match char {
            b'[' => Token::LBrack(usize::MAX), // TODO: better default value
            b']' => Token::RBrack(usize::MAX),
            b',' => Token::Input,
            b'.' => Token::Output,
            _ => panic!("Unexpected character {} when tokenizing", char),
        };
        return Some(token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut tokenizer = Tokenizer::new(".-+><[],");
        let expected = vec![
            Token::Output,
            Token::Dec(1),
            Token::Inc(1),
            Token::IncDP(1),
            Token::DecDP(1),
            Token::LBrack(usize::MAX),
            Token::RBrack(usize::MAX),
            Token::Input,
        ];

        for result in expected.iter() {
            let token = tokenizer.next();
            if let Some(token) = token {
                assert_eq!(token, *result);
            } else {
                assert_ne!(token, None);
            }
        }
    }

    #[test]
    fn test_repeatable() {
        let mut tokenizer = Tokenizer::new("+-->>><<<<");
        let expected = vec![
            Token::Inc(1),
            Token::Dec(2),
            Token::IncDP(3),
            Token::DecDP(4),
        ];

        for result in expected.iter() {
            let token = tokenizer.next();
            if let Some(token) = token {
                assert_eq!(token, *result);
            } else {
                assert_ne!(token, None);
            }
        }
    }

    #[test]
    fn test_repeatable_start() {
        let mut tokenizer = Tokenizer::new("++++");
        let expected = vec![Token::Inc(4)];

        for result in expected.iter() {
            let token = tokenizer.next();
            if let Some(token) = token {
                assert_eq!(token, *result);
            } else {
                assert_ne!(token, None);
            }
        }
    }

    #[test]
    fn test_end_input() {
        let mut tokenizer = Tokenizer::new("++++.");
        tokenizer.next();
        tokenizer.next();
        let token = tokenizer.next();
        assert_eq!(token, None);
        let token = tokenizer.next();
        assert_eq!(token, None);
    }

    #[test]
    fn test_basic_jumps() {
        let tokens = Tokenizer::new("[+-]").tokenize();
        assert_eq!(tokens[0], Token::LBrack(4));
        assert_eq!(tokens[1], Token::Inc(1));
        assert_eq!(tokens[2], Token::Dec(1));
        assert_eq!(tokens[3], Token::RBrack(0));
    }
    #[test]
    fn test_adjacet() {
        let tokens = Tokenizer::new("[]").tokenize();
        assert_eq!(tokens[0], Token::LBrack(2));
        assert_eq!(tokens[1], Token::RBrack(0));
    }
    #[test]
    fn test_nesting() {
        let tokens = Tokenizer::new("[[][[]]]").tokenize();
        assert_eq!(tokens[0], Token::LBrack(8));
        assert_eq!(tokens[1], Token::LBrack(3));
        assert_eq!(tokens[2], Token::RBrack(1));
        assert_eq!(tokens[3], Token::LBrack(7));
        assert_eq!(tokens[4], Token::LBrack(6));
        assert_eq!(tokens[5], Token::RBrack(4));
        assert_eq!(tokens[6], Token::RBrack(3));
        assert_eq!(tokens[7], Token::RBrack(0));
    }
}
