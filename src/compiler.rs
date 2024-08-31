use crate::tokens::Token;

pub fn compile(tokens: Vec<Token>, mem_size: u32) -> String {
    let buff_str = format!("[{:?} x i8]", mem_size);
    let mut result = String::new();

    // setup buffers / llvm
    result.push_str("target triple = \"x86_64-pc-linux-gnu\"\n");
    result.push_str("@.format = constant [2 x i8] c\"%c\"\n");
    result.push_str(&format!("@.buff = global {} zeroinitializer\n", &buff_str));
    result.push_str("@buff = global ptr @.buff\n");
    result.push_str("define i32 @main() {\n");
    result.push_str("%1 = load ptr, ptr @buff\n");

    let mut data_ptr = 1;
    let mut var_count = 1;
    let mut instr_count = 0;
    for token in tokens.iter() {
        match *token {
            Token::Inc(n) => {
                let var_1 = var_count + 1;
                let var_2 = var_count + 2;

                // load at data_ptr -> add -> store back at data_ptr
                result.push_str("\n; increment the value\n");
                result.push_str(&format!("%{:?} = load i8, ptr %{:?}\n", var_1, data_ptr));
                result.push_str(&format!("%{:?} = add i8 %{:?}, {:?}\n", var_2, var_1, n));
                result.push_str(&format!("store i8 %{:?}, ptr %{:?}\n", var_2, data_ptr));

                var_count += 2;
            }
            Token::Dec(n) => {
                let var_1 = var_count + 1;
                let var_2 = var_count + 2;

                // load at data_ptr -> sub -> store back at data_ptr
                result.push_str("\n; decrement the value\n");
                result.push_str(&format!("%{:?} = load i8, ptr %{:?}\n", var_1, data_ptr));
                result.push_str(&format!("%{:?} = sub i8 %{:?}, {:?}\n", var_2, var_1, n));
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
            Token::LBrack(n) => {
                let var_1 = var_count + 1;
                let var_2 = var_count + 2;

                result.push_str(&format!("\n; begin loop"));
                result.push_str(&format!("\nbr label %loop_open_{:?}", instr_count));
                result.push_str(&format!("\n\nloop_open_{:?}:", instr_count));
                result.push_str(&format!("\n%{:?} = load i8, ptr %{:?}", var_1, data_ptr));
                result.push_str(&format!("\n%{:?} = icmp eq i8 %{:?}, 0", var_2, var_1));
                result.push_str(&format!(
                    "\nbr i1 %{:?}, label %loop_end_{:?}, label %loop_body_{:?}",
                    var_2,
                    n - 1,
                    instr_count
                ));
                result.push_str(&format!("\n\nloop_body_{:?}:", instr_count));

                var_count += 2;
            }
            Token::RBrack(n) => {
                let var_1 = var_count + 1;
                let var_2 = var_count + 2;

                result.push_str(&format!("\n%{:?} = load i8, ptr %{:?}", var_1, data_ptr));
                result.push_str(&format!("\n%{:?} = icmp ne i8 %{:?}, 0", var_2, var_1));
                result.push_str(&format!(
                    "\nbr i1 %{:?}, label %loop_body_{:?}, label %loop_end_{:?}",
                    var_2, n, instr_count
                ));
                result.push_str(&format!("\n\nloop_end_{:?}:", instr_count));

                var_count += 2;
            }
            Token::Output => {
                let var_1 = var_count + 1;
                let var_2 = var_count + 2;

                result.push_str("\n; print under data pointer\n");
                result.push_str(&format!("%{:?} = load i8, ptr %{:?}\n", var_1, data_ptr));
                result.push_str(&format!(
                    "%{:?} = call i32 (ptr, ...) @printf(ptr @.format, i8 %{:?})\n",
                    var_2, var_1
                ));

                var_count += 2;
            }
            Token::Input => {
                let var_1 = var_count + 1;
                let var_2 = var_count + 2;

                result.push_str("\n; read a character from stdin");
                // TODO: this may have odd behaviour for EOF / error
                result.push_str(&format!("\n%{:?} = call i32 @getchar()\n", var_1));
                result.push_str(&format!("\n%{:?} = trunc i32 %{:?} to i8", var_2, var_1));
                result.push_str(&format!("\nstore i8 %{:?}, ptr %{:?}", var_2, data_ptr));

                var_count += 2;
            }
        }
        instr_count += 1;
    }

    result.push_str("ret i32 0\n");
    result.push_str("}\n");
    result.push_str("declare i32 @printf(ptr noundef, ...)\n");
    result.push_str("declare i32 @getchar()\n");
    return result;
}
