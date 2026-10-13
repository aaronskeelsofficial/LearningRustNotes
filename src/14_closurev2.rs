fn main() {
    /*
     *  The first closure document explains kind of like a brief overview that gives an idea of what's happening.
     *  The goal of this second iteration is to actually nail down nuance a bit more.
     *  Firstly, a closure is not *technically* "just a function that captures context of scope" like the first document suggests.
     *      A closure, more technically, is sugared syntax that desugars into a full on struct and impl (Fn/FnOnce/FnMut) breakdown.
     *      You write the function, the compiler analyzes what variables are used and in what capacity, it generates the struct,
     *      it initializes the struct and loads the values into it, and it generates the impl function that uses those struct fields.
     *  For Example: */
    let mut counter = 0;
    let mut inner = || { counter += 2; };
    //Is actually
    struct __Closure<'a> { counter: &'a mut i32 }   // one field per capture
    impl<'a> FnMut<()> for __Closure<'a> {
        fn call_mut(&mut self, ...) { *self.counter += 2; }
    }

    /*
     *  The intermingling of functions (fn) and closures (||) is probably of interest. You should know neither is "secretly a
     *  desugared version of the other", as they both truly are distinct. Where there IS overlap though, as you could probably
     *  feel there necessarily is SOME, is that both of them implement the Fn traits. A function is a zero size value wherein
     *  each gets its own unique type signature and it impl Fn functionality. A closure is the struct/impl as shown above.
     */

    /*
     *  Now I'd like to reference the desugaring that goes on with Fn types. Let's say we have */
    fn apply<F>(func: F, value: i32) -> i32
    where
        F: Fn(i32) -> i32, // F must implement `Fn`
    {}
    /*
     *  Typically our "where" clauses can specify what trait TYPE it requires, and maybe generics inside of <>.
     *  Yet this syntax has parenthesis and an arrow notation. What the hell is going on here? This gets desugared behind the scenes is what.
     *  "Fn(i32) -> i32" becomes desugared to "Fn<(i32,), Output = i32>". Now you might be thinking what the hell is fricken even that?
     *  Let's break it down simply. It is saying Fn<INPUT_TYPE, Output = OUTPUT_TYPE>. There are comments to be made on both input/output.
     *  As for the input type, why does it have parenthesis and a random comma?
     *      Parenthesis can be talking about the unit value "()" in the language, OR depending on location they can be talking about a tuple type.
     *      A tuple type is fundamental to the language and should be accepted moreso than "understood deep enough to create it yourself".
     *      A tuple type is a SINGLE type that holds multiple type parameters within it. "(i32,i32,u32)" is a SINGLE type parameter.
     *      The other oddity is the comma. If you attempt to make a single tuple, (i32) for example, it will transform instead into i32 (not a tuple).
     *      If you wish to have a single valued tuple, you must get hacky. Rare, but it does happen in Rust. You need the comma after the type to
     *      preserve the tuple integrity without it decomposing into a normal type outside of a tuple.
     *      So that should explain "Fn<(i32,)," as far as the first argument is concerned.
     *  Now the output type. Why does it look like that? Why is it named? What's the difference?
     *      Firstly, it looks like that because it is NOT a generic. It is handled within the generic syntax location, but it is not a generic.
     *      The Fn trait has something akin to */
            trait FnOnce<Args> {
                type Output;
                // ...
            }
    /*      This is an associated type and we set it within the generic "<>" but it is not a generic.
     *      "If we set it within the generic location and it is a type like a generic, why is it not just a generic" you may ask.
     *      Each input of a generic produces a unique, distinct implementation of the base object. If we allow something like:
     *      "FnOnce<(i32,), String>" AND "FnOnce<(i32,), i32>", then that is not deterministic relative to the intended input types.
     *      We aim to define functions such that the implementation depends on the input types, and the output type is set in stone.
     *      We can't have sometimes we plug in an integer and we get a String and other times we plug in an integer and randomly get an integer.
     *      Therefore, the output type can NOT be a generic (as generics are what define distinct implementations).
     *      The output type must be DERIVED (dependent on, not independently configured) from the input arguments.
     *      This is why it uses an associated type as opposed to generics.
     */

    /*
     *  The final concept is "move" operations. By default, the compiler assumes you want to use the least involved approach possible for your
     *  closures. If you can get by using only a reference, it will default to Fn. If you can get by without consuming anything, it will use FnMut.
     *  Only if you must consume something will it use FnOnce. In short, the default is the least authoritative. The "move" keyword acts to completely
     *  flip this default to shoot for consuming EVERYTHING no matter how you interact with the information within the closure itself.
     *  If you need the data and the operations on that data to superceed the lifetime of the frame they're defined in (perhaps moving to a new thread),
     *  you use "move".
     */
}