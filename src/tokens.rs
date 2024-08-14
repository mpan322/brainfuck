use core::panic;
use std::usize;

pub enum Token {
    Inc(u32),
    Dec(u32),
    IncDP(usize),
    DecDP(usize),
    LBrack,
    RBrack,
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

impl Tokenizer {
    fn is_done(&self) -> bool {
        return self.done;
    }

    fn next_char(&self) -> bool {
        let next_char = self.input.get(self.idx);
        match next_char {
            Some(v) => {
                self.curr = self.next;
                self.next = *v;
                self.idx += 1;
            }
            None => {
                self.next = 0;
                self.curr = self.next;
                if self.curr == 0 {
                    self.done = true
                }
            }
        }
        return self.done;
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

        // condense sequences of characters which are repeatable into 1 token
        if is_repeatable(char) {
            let mut count: u32 = 0;
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
            b'[' => Token::LBrack,
            b']' => Token::RBrack,
            b',' => Token::Input,
            b'.' => Token::Output,
            // ignore all other tokens
        };
        return Some(token);
    }
}
