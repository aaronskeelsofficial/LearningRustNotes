fn main() {
    let x = 5;
    let y = x;
//    Primitives copy by value
//    (a unique spot in memory contains the same value)
    let s1 = String::from("hello");
    let s2 = s1;
//    Whereas non-primitives copy by reference, *and invalidate the original variable*
//    (the same spot in memory is referenced by both variables, but more like transferred)

    fn transfer_ownership(x: String) {
    }
    let x: String = String::from("hello");
    println!("{x}"); //hello
    transfer_ownership(x);
    //println!("{x}"); //ERROR
//    For non-primitives (things without the Copy trait), upon passing them to functions, ownership is transferred
//    Upon falling out of scope in the new function transferred to, memory is dropped and it is inaccessible in the original scope
//    Ownership COULD be transferred back by "let" and "return" usage, but that's silly and stupid. Use references instead.

//    You can either have unlimited immutable references "at the same time"* or one mutable reference
//    You can not return a reference to a variable owned by the child scope to a parent scope or you'll have a hanging reference.
}