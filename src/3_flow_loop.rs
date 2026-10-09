fn main() {
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
//    Note: Only "loop" blocks can return a value as opposed to "while" and "for" loops

    let mut x = 0;
    while x != 42 {
        x += 1;
    }
//    While loop example
}