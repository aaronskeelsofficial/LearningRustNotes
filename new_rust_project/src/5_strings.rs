fn main() {
    let a = "blah";
    let a: &str = "blah";
//    One say to define strings. Immutable and with no ownership. Can't be appended.
    let b: String = String::from("blah");
//    Mutable with ownership. Can be modified.
}