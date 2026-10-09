fn main() {
    struct SeaCreature {
        animal_type: String,
        name: String,
        arms: i32,
        legs: i32,
        weapon: String,
    }
    let ferris = SeaCreature {
        animal_type: String::from("crab"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };
//    Typical struct usage

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
//    Struct init shorthand

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
//    Struct "update"" init shorthand. Copies the value from a pre-existing variable

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let y = origin.1;
//    Tuple structs use the type safety of structs without needing to name every value stored
    
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
//    All struct methods begin with first parameter "self: &Self" which can be shorthanded by "&self"

    struct Rectangle {
        width: u32;
    }
    impl Rectangle {
        fn width(&self) -> u32{
            width
        }
    }
    let rect1 = Rectangle {
        width: 3
    }
//    You can use the same names for variables and methods
    
    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
//    Java static class functions are called "associated functions" in Rust and are called via "STRUCT::FUNCTION()" notation
//    If "Self" is passed in as the first parameter, it is a method that uses dot notation. If it is not, then it is an associated function with double semicolon.

    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let faa = Foo { x: (1, 2), y: 3 };
    let Foo { x : x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");
    // Destructuring works with nested structs as well:
    struct Bar {
        foo: Foo,
    }
    let bar = Bar { foo: faa };
    let Bar { foo: Foo { x: nested_x, y: nested_y } } = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
//    Example of destructuring a struct into component variables
    
    pub struct MyStruct {
        field: i32,
    }
    impl MyStruct {
        // Factory method
        pub fn create(value: i32) -> Self {
            MyStruct { field: value }
        }
    }
//    By having a public struct with private fields inside, we prevent direct instantiation. The factory method MUST be used to create new versions.
    
}