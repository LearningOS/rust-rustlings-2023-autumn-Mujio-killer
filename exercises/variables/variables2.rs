// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    // 1. 不指定类型，仅初始化，会自动根据值判断类型
    // let x = 0;
    // 2. 指定类型1，并且初始化
    // let x:i32 = 0;
    // 3. 指定类型2，并且初始化
    // let x = 0i32;
    // 4. 指定类型3，并且初始化，可读性更好
    let x = 0_i32;

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }

}
