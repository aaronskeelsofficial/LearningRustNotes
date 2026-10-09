fn main() {
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

    for c in "hello".chars() {
        let c: char = c;
    }
//    There is no way to declare types explicitly in the for loop definition itself, so to "declare" types do it like this
//    Technically the type is still inferred for the for loop functionally, but for developer annotations this helps.

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
}