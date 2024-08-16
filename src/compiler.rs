use crate::tokens::Token;

pub fn compile(tokens: Vec<Token>, mem_size: u32) {
    let buff_str = format!("[i8 x {:?}]", mem_size);

    let mut result = String::new();
    let mut dp_num = 1;
    let mut var_count = 1;
    for token in tokens.iter() {
        match *token {
            Token::Inc(n) => {
                let tmp_1_id = var_count + 1;
                let tmp_2_id = var_count + 2;
                let op = format!(
                    "
                  ; inc
                  %{:?} = load i8, ptr %{:?}\n
                  %{:?} = add i8 {:?}, %{:?}\n
                  store i8 %{:?}, ptr %{:?}\n
                ",
                    tmp_1_id, dp_num, tmp_2_id, n, tmp_1_id, tmp_2_id, dp_num,
                );
                var_count += 2;
                result.push_str(&op);
            }
            Token::Dec(n) => {
                let tmp_1_id = var_count + 1;
                let tmp_2_id = var_count + 2;
                let op = format!(
                    "
                  ; dec
                  %{:?} = load i8, ptr %{:?}
                  %{:?} = sub i8 {:?}, %{:?}
                  store i8 %{:?}, ptr %{:?}
                ",
                    tmp_1_id, dp_num, n, tmp_2_id, tmp_1_id, tmp_2_id, dp_num,
                );
                var_count += 2;
                result.push_str(&op)
            }
            Token::IncDP(n) => {
                let op = format!(
                    "%{:?} = getelementptr {}, ptr %{:?}, i64 0, i32 {:?} ; inc dp",
                    var_count + 1,
                    buff_str,
                    dp_num,
                    n
                );
                dp_num += var_count;
                var_count += 1;
                result.push_str(&op);
            }
            Token::DecDP(n) => {
                let op = format!(
                    "%{:?} = getelementptr {}, ptr %{:?}, i64 0, i32 -{:?} ; dec dp",
                    var_count + 1,
                    buff_str,
                    dp_num,
                    n
                );
                dp_num += var_count;
                var_count += 1;
                result.push_str(&op)
            }
            Token::LBrack(_) => (),
            Token::RBrack(_) => (),
            Token::Output => (),
            Token::Input => (),
        }
    }
    println!("{}", result);
}
