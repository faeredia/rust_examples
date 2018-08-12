//This example demonstrates a simple lifetime concept in rust.
//the struct Person is created with a reference toa str.
//The memory for str is assigned elsewhere, we need a way of telling
//the borrow checker that the freshly created structs lifetime
//may not exceed the lifetime of the str.
//Note that for the Person impl, <'a> is needed for both the 
//impl and struct name declarations... a bit ugly.

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

impl <'a> Person <'a>{
    fn greet(&self) { println!("Hello, my name is {}", &self.name); }
}

fn main() {
    println!("Rust lifetime example!");

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{} is {} years old", peter.name, peter.age);
    println!("{:?}", peter);

    peter.greet();
}
