fn main() {
    println!("Hello, world!");
}

// run "cargo new" to initialize a rust project
// you can run "cargo new --vcs=none" to init without a version control system

// all source files are located in the "src" folder
// everything else is one level above

// cargo specifics:
// cargo = cross-system dependency manager and build system for rust
// its basically one tool that handles ur project (cool)

// "cargo build" compiles a development build of the project
// (unoptimized + debug symbols) to help with debugging
// the resulting binary is stored in "target/debug"
// Cargo.lock gets generated on the first build
// it keeps track of the exact versions of dependencies in your project

// "cargo run" compiles the program and runs it right after
// compilation has finished. Its more convinent than building and running
// the binary manually

// "cargo check" analyisis your code to see if it would even compile.
// its a faster way to check your code, than compiling. It should be ran
// perodically (consistently) while writing code.

// "cargo build --release" compiles an optimized build of the project. It does
// take longer to compile though. It is also harder to debug, because of the
// missing debug symbols.

// as your projects grow bigger, youll get a lot more value out of using
// cargo. Its not really necessary for small one-file projects.
// If you want to clone and build a rust project from github, you just have to
// run "cargo build" and your done. Thats actually amazing compared to other
// langauges.
