fn main() {
    let x = 42;
    match x {
        0 => {
        }
        // we can match against multiple values
        1 | 2 => {
        }
        // we can match against ranges
        3..=9 => {
        }
        // we can bind the matched number to a variable
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // this is the default match that must exist if not all cases are handled
        _ => {
        }
    }
//    In Rust, ONLY ONE BRANCH RUNS unlike switch. If multiple are true, the FIRST RUNS ONLY.
//    Multiple conditions and iterative ranges can lead to the same branch
//    You can also bind the value to a variable if you use a range of conditions with "value @ condition =>"

    let triple = (0, -2, 3);
    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (.., 2)  => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        _      => println!("It doesn't matter what they are"),
    }
//    Example of match vector destructuring and syntax

    let array = [1, -2, 6];
    match array {
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
//    Example of match array/slice destructuring and syntax

    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),
        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }
//    Example of match struct destructuring and syntax

    let i = 1;
    match i {
        ref j => {println!("1: {:?}", j)}, // Allowed
        ref mut j => {println!("2: {:?}", j)}, // Allowed
        // &j => {println!("3");}, // Not Allowed
        j => {println!("4: {:?}", j);}, // Allowed
    }
//    Example of using "ref" to not steal ownership of variable in match. By default, if you assign the match arm to variables, you take ownership of them (unless they have Copy trait)

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(String, u16, String), // Takes state, year, and mint mark
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state, _, _) => {
                println!("State quarter from {}!", state);
                25
            }
        }
    }
//    You can bind the INNER data of the matched enum as follows where "_" can be used to "ignore" each piece of data you don't care for. All are required though to bind any.

    enum Enum {
        value(i32),
    }
    let s = Enum::value(5);
    match s {
        Enum::value(v) if v == 5 => {println!("case 1");},
        Enum::value(v) => {println!("case 2");},
        _ => {println!("case 3");},
    }
//    Example of match "guarding" which uses "if" in the match arm. Note: Guarding isn't taken into account when checking match arms cover all possible cases
    
    fn check(value: Option<i32>) {
        match value {
            m @ Some(n @ 10) => println!("Case 1: {} :: {:?}", n, m),
            Some(n @ 6..=9) => println!("Case 2: {}", n),
            Some(n @ 5) => println!("Case 3: {}", n),
            Some(4) => println!("Case 4: {}", 4),
            _ => (),
        }
    }
    check(Some(4));
    check(Some(5));
    check(Some(6));
    check(Some(10));
//    Example of match "binding" done with @. Binding is like simple variable initialization replacements, but it stores the value of ranges so you can know WHAT value matched the condition
    
    fn check_value(x: Option<i32>, value: i32) -> &'static str {
        match x {
            None => "No value",
            Some(i) if i == value => "Matched dynamic value",
            Some(_) => "Matched some other value",
        }
    }
    let five = Some(5);
    let result1 = check_value(five, 5); // Matches dynamic value
    let result2 = check_value(five, 6); // Matches some other value
    let result3 = check_value(None, 5);  // No value
//    Match arms redeclare variables such as "i" in this instance no matter what, shading previous scopes. To dynamically check the value w/o hardcoding, do it like this

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
//    This "if let" pattern imitates match

    enum MyEnum {
        VariantA(i32),
        VariantB(String),
        VariantC,
    }
    let value = MyEnum::VariantA(42);
    //
    if let MyEnum::VariantA(x) = value {
        println!("Matched VariantA with value: {}", x);
    } else if let MyEnum::VariantB(ref s) = value {
        println!("Matched VariantB with value: {}", s);
    } else if let MyEnum::VariantC = value {
        println!("Matched VariantC");
    } else {
        println!("No match");
    }
    //
    match value {
        MyEnum::VariantA(x) => {
            println!("Matched VariantA with value: {}", x);
        }
        MyEnum::VariantB(ref s) => {
            println!("Matched VariantB with value: {}", s);
        }
        MyEnum::VariantC => {
            println!("Matched VariantC");
        }
        _ => {
            println!("No match");
        }
    }
//    Chaining multiple "if let" with "else" can handle destructuring failures or multiple cases emulating a match block
//    Note: match blocks explicitly demand all cases be handled while "if let" blocks can miss possible outcomes. Bad practice to avoid match, though it is possible.

    // METHOD A
    let mut optional = Some(0);
    loop {
        match optional {
            // If `optional` destructures, evaluate the block.
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
            },
            // Quit the loop when the destructure fails:
            _ => { break; }
        }
    }
    // METHOD B
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
//    While let example showcase
    
}