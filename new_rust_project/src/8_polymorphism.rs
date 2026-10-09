fn main() {
//    Polymorphism can be achieved via "trait objects" or "generics w/ trait bounds"
//    If you (and the compiler) know all the types that will possibly be passed in, then use Generics
//    as this compiles down to a unique version of every function for each argument. If you don't
//    explicitly know every type though, then you must use trait objects "&dyn TYPE".

    fn do_something<T: Copy>(x: T) -> T 
    {
    }
//    Example of generics syntax requiring trait bounds that type T implements trait Copy

    fn do_something<T>(x: T) -> T 
    where
        T: TRAIT_TYPE<Output = T> + TRAIT_TYPE<variable = 2>
    {
    }
//    Example of "where" syntax w/ multiple conditions

    fn combine<T, U>(t: T, u: U)
    where
        T: TraitA,
        U: TraitB,
    {
    }
//    Example of "where" syntax impacting multiple type trait bounds

    fn generic_function<A, B>(single_passed_in_value: A) //Here we say this function uses the generics A and B, but only A is actually provided explicitly in the input arguments
    where
        A: Fn() -> B, //Here the type B is defined by doing analysis of the type A. The calling code never actually explicitly says anything about B.
    {
        println!("{}",std::any::type_name::<A>()); //This is just debug code to get the type into string format
        println!("{}",std::any::type_name::<B>()); //This is just debug code to get the type into string format
    }
    fn passed_in_function() -> i32 { //By saying this function outputs an i32 type, we can later perform analysis on this and use this information
        return 5; //Return meaningless integer
    }
    fn main() {
        generic_function(passed_in_function); //Here we provide the function as an argument. This is the only information we explicitly include in the function call, yet it will produce more automatically.
    }
//    Example of generics where calling code only supplies ONE type, and the rest are deduced via computation/analysis of the first
}