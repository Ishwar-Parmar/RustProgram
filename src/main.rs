// Program to simpli print hello world
/*
format!: write formatted text to String
print!: same as format! but the text is printed to the console (io::stdout).
println!: same as print! but a newline is appended.
eprint!: same as print! but the text is printed to the standard error (io::stderr).
eprintln!: same as eprint! but a newline is appended.
*/
fn main() {
    println!("Hello World!!");
    println!("Hello {0}, My name is {1}. What is your nickname {0}", "Ro", "I");
}
