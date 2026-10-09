fn main() {
    fn example_function (param1: i32, param2: i64, param3: f32) -> f64 {
        return 1f64;
    }
//    Base example of a function that takes in 3 parameters and outputs a single f64 value

    fn multiple_outputs () -> (i8, i16, i32) {
        return (1i8, 2i16, 3i32);
    }
//    Example of outputting multiple values

    fn no_returnv1 () {
        
    }
    fn no_returnv2 () -> () {
        
    }
//    Both of these functions have "no return" which is an empty tuple called a "unit". There is no other way to return nothing. Always is a return.

    fn no_semicolon () -> i32 {
        println!("test");
        println!("I am doing stuff");
        5
    }
//    Functions automatically return the last line if no semicolon is used. You can also return traditionally with "return" though from anywhere.
    
//    Functions can be defined in any order even across different modules which I won't pretend to understand

    fn function_example<F: Fn()>(f: F) {
        f();
    }
    fn passed_function() {
        println!("Test");
    }
    function_example(passed_function);
//    Example of how to pass a function as a argument (pretending it's a closure)

/* All across rust, we see the pattern that "str" is the base object and "&str" is the pointer to the object
 *		Functions break this pattern. There *is* no base object for a function. A function is called a "0 byte function item" because they require no memory beyond compile time.
 *		Each function, upon definition, uniquely represents its own type. Everywhere it's used, it is not referenced but substituted in-line post compile time.
 *		"fn" will always represent a function pointer (there is no base fn object).
 * "let op: fn(i32, i32) -> i64 = foo_bar" represents a type signature of "input two i32, output one i64". Any FUNCTION matching that signature is allowed to be assigned.
 *		Functions are fn, not to be confused with CLOSURES represented by Fn
 * "let c: &dyn Fn(i32) -> i32 = &|x| x + 1" represents a (dynamic) closure. Note the capital F used in Fn.
 *		Closures are akin to functions but they are handled behind the scenes quite differently. Don't worry about that here.
 */
}