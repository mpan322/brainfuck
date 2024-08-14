use std::usize;

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
