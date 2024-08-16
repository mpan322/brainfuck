use crate::tokens::Token;

pub fn add_jumps(tokens: &mut Vec<Token>) -> () {
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
            },
            _ => (),
        }
    }

    if stack.len() > 0 {
        panic!("Unbalanced brackets - too many opening brackets");
    }
}

#[cfg(test)]
mod test {
    use crate::tokens::Tokenizer;

    use super::*;

    #[test]
    fn test_basic() {
        let mut tokens = Tokenizer::new("[+-]").collect();
        add_jumps(&mut tokens);
        assert_eq!(tokens[0], Token::LBrack(4));
        assert_eq!(tokens[1], Token::Inc(1));
        assert_eq!(tokens[2], Token::Dec(1));
        assert_eq!(tokens[3], Token::RBrack(0));
    }
    #[test]
    fn test_adjacet() {
        let mut tokens = Tokenizer::new("[]").collect();
        add_jumps(&mut tokens);
        assert_eq!(tokens[0], Token::LBrack(2));
        assert_eq!(tokens[1], Token::RBrack(0));
    }
    #[test]
    fn test_nesting() {
        let mut tokens = Tokenizer::new("[[][[]]]").collect();
        add_jumps(&mut tokens);
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
