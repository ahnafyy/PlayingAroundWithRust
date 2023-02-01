// This is the main hello world program

/// `println!` is a macro that prints a string to the console
// The body of the main function holds the following code:
//     println!("Hello, world!");
// This line does all the work in this little program: it prints text to the screen.
// There are four important details to notice here.

// First, Rust style is to indent with four spaces, not a tab.

// Second, println! calls a Rust macro. If it had called a function instead, it would be entered as println (without the !).
// We’ll discuss Rust macros in more detail in Chapter 19. For now, you just need to know that using a ! means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions.

// Third, you see the "Hello, world!" string. We pass this string as an argument to println!, and the string is printed to the screen.

// Fourth, we end the line with a semicolon (;), which indicates that this expression is over and the next one is ready to begin. Most lines of Rust code end with a semicolon.

fn main() {
    println!("Hello, world!");
}
