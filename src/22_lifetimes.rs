fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("{r}"); // ERROR
    //The problem with lifetimes is that references living in scope longer than the scope of the object they reference doesn't make sense.

    //Explicitly declaring lifetime in variable
    let r: &'a i32;

    //Explicitly declaring lifetime in function parameter
    fn read<'a>(x: &'a i32) {}

    //Explicitly declaring lifetime in function return
    fn get<'a>(x: &'a i32) -> &'a i32 {}

    //Explicitly declaring lifetime in struct
    struct TextView<'a> {
        text: &'a str,
    }

    //Technically, every single function and variable in Rust is defined with a type and lifetime. Whenever we don't include them, that is magical syntactic sugar that the compiler changes behind the scenes for us. They are NECESSARY.
    // Type Inference assumes the type, if possible, from context clues
    // Lifetime Elision assumes the lifetime, if possible, from context clues
}