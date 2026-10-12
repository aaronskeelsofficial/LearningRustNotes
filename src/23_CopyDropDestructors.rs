fn main() {
    /*
     * Let's recap,
     * A String struct holds a ptr, len, and cap
     *      A String (behind the curtain) also owns a chunk of data on the heap which is what the other info is about
     * Every value has exactly one owner who, when out of scope, drops the value and resources are freed */
        let s: String = String::from("hello"); //Here String s owns the &str "hello" contained on the heap

    //We can perform "Move" operations which transfer ownership, often without copying heap data only copying the stack header
        let s = String::from("hi");
        let t = s; //This copies the stack-located header of s into t, transfers &str ownership to t, and invalidates the usage of s
        // println!("{s}") //Error: use after move

    //We can also perform "Copy" operations which do NOT transfer ownership, instead copying direct stack values outside of headers
        //Note: This is only possible for data with known sizing and which doesn't rely on the heap or necessary cleanup
    #[derive(Copy)]
    struct Point { x: i32, y: i32 }
    let p = Point { x: 1, y: 2};
    let q = p; //p is still valid and alive here with its ownership and original header, everything was copied distinctly not moved

    //When values are being freed and you need a cleanup process, you can opt in to Drop
    struct LogOnDeath(&'static str);
    impl Drop for LogOnDeath {
        fn drop(&mut self) {
            println!("dropping {}", self.0);
        }
    } //Runs automatically when the value dies

    //Note: Copy and Drop can not BOTH exist on an object. This is not a logical necessity, but a chosen enforcement for the following reason
        //Copy communicates "the author does not need any knowledge that the data was duplicated, nothing need be accounted for"
        //Drop communicates "when this data is removed, things definitely need to be accounted for"
    //One COULD safely copy an object and run the same destructor for both, perhaps something like this:
    struct MyType {
        id: i32,
    }
    impl Drop for MyType {
        fn drop(&mut self) {
            println!("Dropping {}", self.id);
        }
    }
    {
        let a = MyType { id: 1 };
        let b = a;
    } // Scope ends, both print lines fire
    //There is no harm in this and the Rust enforced ban of Copy/Drop simultaneously seems silly
    //The reason it exists, is again about what the traits are meant to communicate and the TYPICAL implication that carries.

    //There is something called "drop glue" which happens implicitly without you needing to specify behavior
        //When an object dies, its fields' destructors run automatically and recursively
        //Interesting Note: Everything is dropped in reverse order from definition (variables are removed bottom of code block to top, likely following dependencies)

    //Clone is like copy, though Clone is the more general purpose adaptation.
        //Clone does NOT require all data exist on the stack
        //Clone can copy stack and heap data
        //Copy is implicit (applies automatically on equal operator) while Clone is an intentional function call
    //Note: Every Copy is a subtype of Clone since Clone is the more generic/less restricted implementation

    /*
     * All these subjects have a useful endgame that is mutex/guard locks. The idea is the lock reserves ownership of the data and automatically kills itself
     * the instant it goes out of scope, no need for an arbitrary garbage collector or manually typed memory freeing byt the dev.
     */
}