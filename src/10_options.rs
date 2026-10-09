fn main() {
    enum Option<T> {
        None,
        Some(T),
    }
//    Option enums are built into the language as a way to FORCE good coding behavior regarding handling null checks\

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; //ERROR
//    This code would produce an error because you can not treat a "successful" option value as the original type of that value. You must explicitly handle and convert it.
}