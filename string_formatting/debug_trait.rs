struct Something(i32);

#[derive(Debug)]
struct SomethingWithDebug(i32);

#[derive(Debug)]
struct DebugCeption(SomethingWithDebug);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    // Debug formatting
    println!("{:?}", 42);

    // Does not implemet Debug. Does not compile!
    // println!("{:?}", Something(3));    
    
    let with_debug = SomethingWithDebug(3);
    // Implements Debug
    println!("{:?}", with_debug);

    // Implements Debug
    println!("{:?}", DebugCeption(with_debug));

    // Debug with pretty print
    let name = "Peter";
    let age = 42;
    let peter = Person { name, age };
    println!("{:#?}", peter);
}
