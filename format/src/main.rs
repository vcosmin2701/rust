fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("Base 10:                      {}", 69420);
    println!("Base 2 (binary):              {:b}", 69420);
    println!("Base 8 (octal):               {:o}", 69420);
    println!("Base 16 (hexadecimal):        {:x}", 69420);

    println!("{number:>5}", number = 1); // right-justify
    println!("{number:0>5}", number = 1); // pad with 0
    println!("{number:0<5}", number = 1); // left-adjust

    println!("{number:0>width$}", number = 1, width = 5);

    /* Only types that implement fmt::Display can be formatted
     * with `{}`
    #[allow(dead_code)]
    struct Structure(i32);

    fmt::Display
    println!("This struct `{}` won't print...", Structure(3));
    */

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("{:.3}", pi)
}
