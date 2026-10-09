#[allow(non_snake_case)]

fn main() {
    let x = "test";
    println!("blah blah {}", x); //blah blah test
    println!("blah blah {x}"); //blah blah test
    println!("blah blah {:?}", x); //blah blah "test"
    println!("blah blah {x:?}"); //blah blah "test"
//    Different formats to println. You can shorten standard Display traits, but you can not shorten other custom displays like debug.
//    Display {}, Debug {:?}, Pretty Debug {:#?}

//    dbg!(EXPRESSION);
//    dbg! takes ownership, prints out expression and return, and then returns ownership of return

//    Automatic dereferencing so you can call ".method()" from a reference pointer directly or manual dereferencing (*reference_variable)

//    Result -> Ok(value) or Error(value)
//    Option -> Some(value) or None
//      Note: There is std::result::Result<T, std::io::Error> and std::io::Result<T>. io::Result assumes the error type is std::io::Error.

    let v = {
        let a = 1;
        let b = 2;
        a + b
    };
//    Scope blocks are better versions of anonymous functions from JS that allow you to not pollute the original scope with variables

//    Mutability is achieved via attaching "mut" to the VARIABLE if dealing with the variable directly, and attaching to the TYPE if dealing with a reference to the variable
//    ex:   let mut x: i32 = 2;      let x_ref: &mut i32 = &x;

    #[derive(Debug)]
    struct Struct {
        value: i32,
    }
    impl Struct {
        fn get_ref(&self) -> &Self {
            dbg!(self);
            &self
        }
    }
    let s = Struct { value: 5 };
    let _s_ref = s.get_ref();
//    In rust, there is "Self", "&Self", "self", and "&self"
//    "Self" is a type, and "self" is the value of an instance

//    In Rust, you can either have multiple immutable borrows and no mutable borrows until they're dropped,
//    or you can have a single mutable borrow and no immutable borrows at the same time

//    .unwrap() processes Option<> and Result<> inner values or exits with panic
//    "?" returns inner value or None for Option<>, or returns inner value or propagates error for Result<>
//    Manual match arm handling can use "panic!()"

//    Rc<> enables ownership "ambiguity" where you can have multiple borrowed references of a value whose owner is nebulous in the clouds, and it is dropped upon all reference drops
//    Arc<> is the multithread safe version with extra overhead

    struct MyStruct {
        value: i32,
    }
    impl MyStruct {
        fn new(value: i32) -> MyStruct {
            MyStruct { value }
        }
    
        fn display(&self) {
            println!("Value: {}", self.value);
        }
    }
    fn main() {
        let my_instance = MyStruct::new(42);
        // Calling the method using the instance
        my_instance.display();
        // Calling the method with manual passing
        MyStruct::display(&my_instance);
    }
//    Functions are called "methods" if they are being called assuming an instance (take &self as first argument)
//    Functions without &self as first, are treated like "static" in java
//    Even methods can be called using "associated function" syntax "::" in place of "method" syntax "." if you manually pass in &self argument
//    AKA "." syntax = "::" syntax automating the first argument as the instance reference.

    struct Number {
        value: i32,
    }
    impl std::convert::From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }
//    //From is a commonly used standard trait for converting data types
//    //To is the complimentary trait, but implementing From automates "To" for the opposing data type without direct implementation.
}
