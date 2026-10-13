fn main() {
    /*
     *  In Rust there are "bindings" and there are "values".
     *      A "binding" is a compile time name attached to a place (typically a stack slot) that can hold bytes
     *      A "value" are the bytes plus their meaning (the INTEGER 2, the STRING "hello")
     *  A String/Vec are actually split among two memory locations
     *      Inline (24 bytes total): pointer to heap buffer (8), length (8), capacity (8). Aka a fat pointer.
     *      Heap Buffer: the actual dynamically sized data
     *  Unlike String/Vec, an i32 is only stored in one location and its inline represents the data itself
     *      Inline (4 bytes total): data itself
     * 
     * 
     *  So let's set our mental framework clearly: a binding holds a stack slot in which inline data exists that may or may not reference a heap buffer.
     *  Now we talk about operations on memory which regard this framework highly: move, Copy, and Clone.
     *      move: This memcpy the inline data from binding A to binding B, leaves heap data exactly where it is, transfers *ownership* of any heap data to B, and invalidates binding A.
     *          Note: This does NOT clear, free, destroy, etc the data from binding A. A still holds the inline data and any references to heap buffers.
     *          The language just blocks us from using A in any capacity to enforce single owners, prevent double-free, etc. But the data is still there until it is out of scope/dropped.
     *          Note: There may BE no heap buffer data, therefore no need to invalidate A to prevent double-frees. We must annotate the assurance of no heap buffer by implementing Copy.
     *      Clone: This creates a new binding B, memcpy all inline data from A to B, allocates new heap buffers to match A, memcpy heap buffer from A to B, and both binding A and B are valid.
     *          Note: This is effectively a move, but we are ALSO creating distinct, copied heap data rather than transferring reference and ownership between bindings.
     *          As the most sizeable information is typically found in heaps, a Clone is by far the most expensive of all the operations especially if the object cloned has many heap buffers within.
     *      Copy: A Copy is a move with the compiler analysis assurance that there *is no heap data*, therefore both bindings stay valid since there can't be a double-free.
     * 
     * 
     *  Another note of memory management that seems important is how borrowing actually *works*.
     *  Let's say we have this code */
    let mut z = 5;
    let r = &mut z;   // the token moves: z lends its write permission to r
    z = 10;           // ERROR: cannot use `z` — it is mutably borrowed
    *r += 1;          // fine: r holds the token
    // ... r's last use passes here ...
    z = 10;           // fine again: the loan ended, the token is back
    /*  In this code we make a binding "z" with a compiler annotation that it is mutable (this is not REAL data in any way past compile time, it lives only in the brain of the compiler)
     *  In other assignment code not relating to references, we always will experience a move, Copy, or Clone approach. Which is used in references?... None of them.
     *  References freeze the original binding (via compiler) until said reference is dropped, and create a NEW stack slot holding an inline 8 byte pointer to the original binding's address.
     *  Reference usage/lifetime/ownership rules are ENTIRELY FAKE. They live purely in compiler analysis and absolutely no remnants of said analysis live on through to the compiled binary.
     *  The COMPILER brain keeps track of "who did we give permission to" and "have they given up permission rights via their last usage"
     *  So naturally, the follow up question is "how does the compiler know when the last usage happens"
     *      In short, it makes a control-flow graph and runs analysis on it. In places where it can't PROVE the reference is dead (since it can't simulate unpredictable function calls),
     *      it just assumes it's alive. This can cause many borrow checker headaches where YOU know the code is safe given technicalities of simulating the execution, but the compiler
     *      doesn't know it's safe. Therefore you must PROVE to the compiler its safe, or it assumes it's not and keeps the original binding frozen until the reference dies.
     */
}