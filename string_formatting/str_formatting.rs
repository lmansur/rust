fn main() {
    println!("======= BEGIN EXAMPLES =======");
    // Standard printing
    println!("Hello, world!");

    // Printing with parameters
    println!("{}, {}", "Hello", "world!");

    // Printing with named parameters. The order doesn't matter.
    println!("Hello, {guest}. My name is {host}", host="Lucas", guest="Mike");

    // Printing with numbered parameters.
    println!("Hello, {1}. What did you do {0}?", "today", "Mike");

    // Formats the number leading zeros until the desired width is reached (4)
    println!("The answer is: {:04}", 42);

    // Pads the number with whitespace until the desired width is reached (4)
    println!("The answer is: {:4}", 2);

    // Formats the number with 3 decimal houses
    println!("PI is roughly {:.3}", 3.141592);

    // Prints the number in Binary
    println!("{num} in binary is {num:b}", num=42);

    // Prints the number in Octal
    println!("{num} in octal is {num:o}", num=42);

    // Prints the memory address
    println!("{ref}'s address {ref:p}", ref=&42);

    // Print does not accept variables in place of the format string
    // This fails to compile:
    //   let some_string = "Some string";
    //   println!(some_string);

    println!("======= END EXAMPLES =======");
}
