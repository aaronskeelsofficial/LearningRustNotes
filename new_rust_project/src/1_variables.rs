fn main() {
    let immutableVariable = 1;
//    Immutable variables are made with "let". Auto type with inference

    let typedVariable: i32 = 1;
//    Types can also be explicitly mentioned

    let variable: i32 = 0;
    let variable: i32 = 1;
//    You can initialize the same variable name multiple times?

    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("sum is {}", sum);
//    Mutable variables require "mut"

    let mut var2 = 0.0;
    var2 += 1 as f64;
    println!("var2 is {}", var2);
//    There is no secret casting. All must be explicit.

    for i in 0..5 {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
    }
//    Assigning variables must be done with conditions instead of ternary operator

    let food = "hamburger";
    let result = match food {
        "hotdog" => "is hotdog",
        _ => "is not hotdog",
    };
//    Variables can be set via "match" syntax

    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];
    let slice = &s[3..];
    let slice = &s[..];
//    New Rust data type called "string slice" indicated by &str

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
//    All data types have slices. This would be type &[i32]

/*
 * Rust has three memory sources: heap, stack, and compiled binary
 * 		heap: A String lives on the heap
 *		stack: A fixed array [u8; 4] lives on the stack
 *		compiled binary: A string literal "hello" lives in compiled binary storage
 * Rust has owned types and borrowed slices
 *		owned types: Live on the stack as a manager. It owns its heap allocations and is responsible for allocating, reallocating, and freeing memory.
 *		borrowed slice: This is a window pointing to data owned by someone else (whether that be on heap, stack, or compiled binary).
 * Rust has normal pointers and fat pointers
 *		normal pointers are 8 bytes and simply point to a memory location
 *		fat pointers are 16 bytes and contain an 8 byte normal pointer to a memory location and an 8 byte length parameter (can represent bytes or elements)
 * Rust requires known sizing on ALL stack information by compile time. A "str" is an example of a dynamically sized type meaning its size isn't known at compille time.
 *		to work around this, we must pass &str in place of str for function arguments
 * Coming from another programming language, one's mental model may be akin to:
 * 		The raw data itself is bytes in memory <- We wrap those bytes into a representational object/class <- A pointer references the object/class wrapped around the data
 * In Rust, your mental model must shift to accept the unique difference that *primitive types ARE the data*
 *		The raw data itself is bytes in memory, and this data *is the primitive type* such as `str` <- &str is a fat pointer with address/length information on that data
 * In Java, different types carry through to runtime. The objects physically are treated different behind the scenes.
 *		In Rust, typing is a compiler sanity/validity check and at runtime, where possible, there is zero-cost abstraction meaning those raw bytes can be called str or [u8] equally for free.
 * Rust has two string types (among others) and their difference helps demonstrate multiple memory concepts
 *		String holds ptr/cap/len, is sized, and as such lives on the stack. String acts as the owner/manager of the memory of a str object.
 *		str IS the byte data itself, is of unknown size (as it can be any length), and lives in any possible memory region.
 * A str can NEVER own the data. A str is a semantic label placed over data owned by another object that handles owning/managing memory
 *		stack owned: We can construct a &str from "[u8; 5] = [104, 101, 108, 108, 111];"
 *		heap owned: We can construct a &str from "String = String::from("hello");"
 *		compiled binary owned: We can construct a &str from `&'static str = "hello";`
 * Rust typically actually uses three typical string implementations in code
 *		&str: This looks at data owned elsewhere and enforces str rules on top of it
 *		String: This owns/manages the data of a str inside of it, and gives headroom for easy expansion without constant full-copy reallocation
 *		Box<str>: This owns/manages the data of a str inside of it, yet gives no headroom for expansion. It is used to "lock in" text ineditably taking up less space than String data.
 *			This takes less space because it doesn't need a capacity field in its core struct, and it also holds no extra headroom in memory.
 */
{