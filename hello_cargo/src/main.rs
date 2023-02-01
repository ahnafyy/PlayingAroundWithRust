/// The main function is the entry point of the program
fn main() {
    println!("Hello, Cargo 📦! ");
}

// I can run cargo build to compile the code and cargo run to compile and run the code
// $ cargo build

// This generates a binary file in the target/debug directory. You can run this file directly:
// $ ./target/debug/hello_cargo

// You can also use cargo run to compile and run your code:
// $ cargo run

// This command compiles your code, creates an executable, and runs it. The output should be the same as when you ran the executable directly:

// I can also run cargo check to check whether the code compiles without actually producing an executable:

// $ cargo check

// This command is useful if you want to check whether your code compiles but don’t need the executable. It runs much faster than cargo build because it doesn’t produce an executable.

// I can also run cargo build --release to compile the code in release mode, with optimizations:

// $ cargo build --release
