fn main() {
    // println! is called: macro
    println!("Hello, world!");

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}

// Step to generate binary: rustc main.rs
// Create folder with essentials: cargo new name

/*
 * Multi-line comments can be written like this
 * Worth to mention: /* disable blocks of functionality inline */
 */
