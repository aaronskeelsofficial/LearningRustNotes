//    Fundamentally, closures are just functions which intake the context of their scope and can modify it

let closure_annotated = |i: i32| -> i32 { i+1 };
let closure_inferred = |i| i+1;
//    Closures can have heavily annotated or inferred syntaxes

fn outer_function() {
    let mut counter = 0;
    // The following function yields an error.
    // fn inner_function(x: i32) -> i32 {
    //     counter += x;
    //     counter
    // }
    
    let mut inner_closure = |x| {
        counter += x;
        counter
    };

    // let function_result = inner_function(5); //Error
    let closure_result = inner_closure(5);
    println!("Results: {:?}", closure_result);
}
fn main() {
    outer_function();
}
//    This example shows how functions can't touch environmental data, but closures can.

fn apply<F>(func: F, value: i32) -> i32
where
    F: Fn(i32) -> i32, // F must implement `Fn`
{
    func(value)
}
fn apply_mut<F>(func: F, value: i32) -> i32
where
    F: FnMut(i32) -> i32, // F must implement `FnMut`
{
    func(value)
}
fn apply_once<F>(func: F)
where
    F: FnOnce(), // F must implement `FnOnce`
{
    func()
}
//    Above are examples of how to define Trait Bounds for functions requiring certain kinds of closures.
//    Fn: Can be called multiple times without changing state, captures environment by reference
//    FnMut: Can be called multiple times but does change state, captures environment by mutable reference
//    FnOnce: Can only be called once, captures environment by transferred ownership. Useful for preserving values when original scope is no longer needed (ex: starting new threads)
//    Note: Actual functions can be passed in in place of closures, but all actual functions require "Fn" syntax.

fn main() {
    let data = String::from("Hello, Thread!");

    let handle = thread::spawn(move || {
        println!("{}", data); // `data` is moved into the thread
    });

    handle.join().unwrap();
}
//    Example of "move" keyword, which transfers ownership of variables used within closure to the closure. Useful to make them outlive their original scope. (ex: starting threads)