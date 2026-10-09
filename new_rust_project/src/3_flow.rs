fn main() {
    let x = 42;
    if x < 42 {
        println!("less than 42");
    } else if x == 42 {
        println!("is 42");
    } else {
        println!("greater than 42");
    }
//    Conditionals don't need parenthesis and actually yell if you use them (but you can)

//    Conditional operators are ==,!=,<,>,<=,>=,!,||,&&

    for i in 0..5 {
    }
//    For loops don't need parenthesis but need braces, and go across iterators
    for x in 0..5 {
    }
    for x in 0..=5 {
    }
//    .. iterator includes the lower bound but does not include the upper
//    ..= iterator includes the lower bound and the upper
    for x in (0..=5).rev() {
    }
//    Ranges can not be decreasing. To decrease, you must reverse the increase.

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);
    //-
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
    //-
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);
    // FIXME ^ Comment out this line
//    for ".iter()" syntax means you are borrowing the elements each loop
//    for ".iter_mut()" syntax means you are borrowing the elements mutably each loop
//    for ".into_iter()" syntax means that you are transferring ownership on the iteration and the original variable loses ownership

    let mut x = 0;
    loop {
        x += 1;
        if x == 42 {
            break;
        }
    }
//    Infinite loop without condition can be broken by "break" inside

    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "found the 13";
        }
    };
//    Loops can return a value if placed after "break"

    'outer_loop: loop {
        loop {
            break 'outer_loop;
        }
    }
//    Nested loops can be controlled via loop tags done with a single quote

    let result = 'outer: loop {
        let mut count = 0;
        'inner: loop {
            count += 1;
            if count == 3 {
                break 'outer count * 2;
            }
        }
    };
//    Example of returning value along with loop label usage

    let mut x = 0;
    while x != 42 {
        x += 1;
    }
//    While loop example

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
//    This "if let" pattern imitates
}