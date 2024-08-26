use crate::tokens::Token;

pub fn compile(tokens: Vec<Token>, mem_size: u32) {
    let buff_str = format!("[{:?} x i8]", mem_size);

    let mut result = String::new();

    // setup buffers / llvm
    result.push_str("target triple = \"x86_64-pc-linux-gnu\"\n");
    result.push_str("@.format = constant [4 x i8] c\"%c\\0A\\00\"\n");
    result.push_str(&format!("@.buff = global {} zeroinitializer\n", &buff_str));
    result.push_str("@buff = global ptr @.buff\n");
    result.push_str("define i32 @main() {\n");
    result.push_str("%1 = load ptr, ptr @buff\n");

    let mut data_ptr = 1;
    let mut var_count = 1;
    for token in tokens.iter() {
        match *token {
            Token::Inc(n) => {
                let var_1 = var_count + 1;
                let var_2 = var_count + 2;

                // load at data_ptr -> add -> store back at data_ptr
                result.push_str("\n; increment the value\n");
                result.push_str(&format!("%{:?} = load i8, ptr %{:?}\n", var_1, data_ptr));
                result.push_str(&format!(
                    "%{:?} = add nuw i8 {:?}, %{:?}\n",
                    var_2, n, var_1
                ));
                result.push_str(&format!("store i8 %{:?}, ptr %{:?}\n", var_2, data_ptr));

                var_count += 2;
            }
            Token::Dec(n) => {
                let var_1 = var_count + 1;
                let var_2 = var_count + 2;

                // load at data_ptr -> sub -> store back at data_ptr
                result.push_str("\n; decrement the value\n");
                result.push_str(&format!("%{:?} = load i8, ptr %{:?}\n", var_1, data_ptr));
                result.push_str(&format!(
                    "%{:?} = sub nuw i8 {:?}, %{:?}\n",
                    var_2, n, var_1
                ));
                result.push_str(&format!("store i8 %{:?}, ptr %{:?}\n", var_2, data_ptr));

                var_count += 2;
            }
            Token::IncDP(n) => {
                let var = var_count + 1;

                result.push_str("\n; add to data pointer\n");
                result.push_str(&format!(
                    "%{:?} = getelementptr {}, ptr %{:?}, i64 0, i32 {:?}\n",
                    var, buff_str, data_ptr, n
                ));

                data_ptr = var;
                var_count += 1;
            }
            Token::DecDP(n) => {
                let var = var_count + 1;

                result.push_str("\n; subtract from data pointer\n");
                result.push_str(&format!(
                    "%{:?} = getelementptr {}, ptr %{:?}, i64 0, i32 -{:?}\n",
                    var, buff_str, data_ptr, n
                ));

                data_ptr = var;
                var_count += 1;
            }
            Token::LBrack(_) => (),
            Token::RBrack(_) => (),
            Token::Output => {
                let var_1 = var_count + 1;
                let var_2 = var_count + 2;

                result.push_str("\n; print under data pointer\n");
                result.push_str(&format!("%{:?} = load i8, ptr %{:?}\n", var_1, data_ptr));
                result.push_str(&format!("%{:?} = call i32 (ptr, ...) @printf(ptr @.format, i8 %{:?})\n", var_2, var_1));

                var_count += 2;
            }
            Token::Input => {}
        }
    }

    result.push_str("ret i32 0\n");
    result.push_str("}\n");
    result.push_str("declare i32 @printf(ptr noundef, ...)\n");
    println!("{}", result);
}
