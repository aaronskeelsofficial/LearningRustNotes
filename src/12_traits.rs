fn main() {

    //A trait describes a capability that a type can provide and looks like
    trait Fly {
    }

    //Traits can have associated types and associated functions, but not associated variables. These are templates defined on a case by case basis in accordance to specific "impl" blocks
    trait Fly {
        type foo;
        fn fly(&self);
    }

    //Traits can have default function implementations defined within their "trait" definition rather than an "impl" block
    trait Fly {
        fn fly(&self) {
            //do something
        }
    }

    //Traits are impl on top of structures
    struct Bird;
    trait Fly {
        fn fly(&self);
    }
    impl Fly for Bird {
        fn fly(&self) {
            //...
        }
    }

    //If you override the default behavior, there *is no way to explicitly execute the default ever again*
    struct Bird;
    trait Fly {
        fn fly(&self) {
            println!("A");
        }
    }
    impl Fly for Bird {
        fn fly(&self) {
            println!("B");
        }
    }
    let bird = Bird;
    bird.fly(); //This will print B
    <Bird as Fly>::fly(&bird); //This will print B

    //We can require input parameters that implement a certain trait as such (thing must implement Fly trait)
    fn take_off<T: Fly>(thing: &T) {}
    //or the shorthand version
    fn take_off(thing: &impl Fly) {}

    //We can require return types that implement a certain trait, though this approach does NOT use generics (since generics revolve around input types and the output is defined). The return type is deduced concretely by analyzing the code within the function. Its use is kinda annotative, though there is a function utility as it acts as an interface not requiring refactoring when code changes internally.
    fn create_flyer() -> impl Fly {
        Bird;
    }

    //Because the function requires a concrete typing, analysis MUST yield a single return type so the following is not allowed
    fn create_flyer() -> impl Fly {
        if cond {
            Bird //This isn't allowed, must return ONE type
        } else {
            Airplane //This isn't allowed, must return ONE type
        }
    }
}