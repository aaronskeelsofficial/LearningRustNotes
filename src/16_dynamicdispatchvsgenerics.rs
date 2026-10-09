//    There are two approaches for allowing polymorphism: Generics and Dynamic Dispatch
//    The key difference is Generics handle polymorphism at compile time whereas Dynamic Dispatch handles polymorphism at runtime w/ overhead

//    Generics require all possible overloaded types be known at compile time, and a function is literally compiled with duplicates for each unique type
//    Dynamic Dispatch allows for external users to build on top of your existing code, adding new types you didn't know of at compile time

//    Generics look like the following:
    fn generic_function<T>(t: T) {
        //DO SOMETHING
    }
//    Dynamic Dispatch looks like the following:
    trait Shape {
        fn area(&self) -> f64;
    }
    struct Circle {
        radius: f64
    }
    struct Square {
        length: f64
    }
    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }
    impl Shape for Square {
        fn area(&self) -> f64 {
            self.length * self.length
        }
    }
    fn print_area(s: &dyn Shape) {
        println!("Area: {}", s.area());
    }
    let circle = Circle { radius: 5.0 };
    let square = Square { length: 5.0 };
    print_area(&circle);
    print_area(&square);