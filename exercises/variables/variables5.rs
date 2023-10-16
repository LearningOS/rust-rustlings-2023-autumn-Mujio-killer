// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // 遮蔽原变量number
    let mut number = 1;
    number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
