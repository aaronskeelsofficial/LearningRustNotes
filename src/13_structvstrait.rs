fn main() {

//    To begin, know that there are structs vs traits.
//    There are associated variables, constants, functions, and types
//    There are methods (functions which bind to, and are called by, a specific instance)
//    There are the struct, trait, struct impl, and trait impl blocks



/*    
*     Structs Allow:
*       Def:
*         Associated Variables: Abstract
*       Impl:
*         Associated Constants: Defined
*         Associated Functions: Defined
*         Associated Types: Defined
*         Methods: Defined
*/
/*
*     Traits Allow:
*       Def:
*         Associated Constants: Abstract (inherited) or Defined (default)
*         Associated Functions: Abstract (inherited) or Defined (default)
*         Associated Types: Abstract
*         Methods: Abstract (inherited) or Defined (default)
*       Impl:
*         Associated Constants: Only if exists in def
*         Associated Functions: Only if exists in def
*         Associated Types: Only if exists in def
*         Methods: Only if exists in def
*/
/*
*     In Other Words, The Following Are Allowed In These Places (abstract/defined/special):
*       Associated Variables: Struct Def (ab)
*       Associated Constants: Struct Impl (def), Trait Def (ab/def), Trait Impl (*)
*       Associated Functions: Struct Impl (def), Trait Def (ab,def), Trait Impl (*)
*       Associated Types: Struct Impl (def), Trait Def (ab), Trait Impl (*)
*       Methods: Struct Impl (def), Trait Def (ab/def), Trait Impl (*)
*/
/*
*     In paraphrased English words:
*     Struct Def is for a blank template of VARIABLES (data) to be filled in
*     Struct Impl is primarily for adding FUNCTION/METHOD functionality, but you can also add constants and types on the side
*     Trait Def is for creating a blank template of FUNCTIONS/METHODS to be filled in which act on the variables found in a struct,
*       but you can also add abstract constants and types on the side
*     Trait Impl is primarily for specifically implementing the FUNCTIONS/METHODS/CONSTANTS/TYPES outlined in the template of Trait Def
*       with consideration to the struct the trait is being added to.
*     Structs are for one time use with no planned "extensions" building off of them aside from prepackaged functionality inherited from traits
*     Traits are used for inheritance purposes and polymorphism
*/



//    Struct defs have ONLY Associated Variables separated by commas with no initialization
//    Associated Variables can NOT have default values
    struct StructExample {
        //Associated Variable (Must be abstract)
        abstract_associated_variable: f64,
    }
    
    
    
//    Struct impl blocks allow associated constants, functions, types, and methods separated by normal semicolon syntax
//    All of these must be defined
    impl StructExample {
        //Associated Type (Must be defined)
        // type DefinedAssociatedType = i32; //Error - Should work but is currently an unstable feature
        
        //Associated Constant (Must be defined)
        const DEFINED_ASSOCIATED_CONSTANT: i32 = 1;
        //Note: It is constant rather than variable because of "const" rather than "let"
        
        //Associated Function (Must be defined)
        fn defined_associated_function(radius: f64) -> Self {
            unimplemented!();
        }
        
        //Method (Must be defined)
        fn defined_method(&self) -> Self {
            unimplemented!();
        }
    }
    
    
    
//    Trait defs allow associated constants, functions, types, and methods separated by normal semicolon syntax
//    All CAN have default values except for Associated Types which are specified in each impl block
    trait TraitExample {
        //Associated Type (Must be abstract)
        type AbstractAssociatedType; //Note: You MUST specify this in the impl if you define it abstract
        
        //Associated Constant (Can be abstract or defined aka default)
        const ABSTRACT_ASSOCIATED_CONSTANT: i32; //abstract
        const DEFINED_ASSOCIATED_CONSTANT: i32 = 1; //defined
        //Note: coming from java this uninitialized "const" behavior is sacrilegious, but in Rust const only applies AFTER first init.
        
        //Associated Function (Can be abstract or defined aka default)
        fn abstract_associated_function();
        fn defined_associated_function(radius: f64) -> Self::AbstractAssociatedType {
            unimplemented!();
        }
        
        //Method (Can be abstract or defined aka default)
        fn abstract_method(&self);
        fn defined_method(&self) -> Self::AbstractAssociatedType {
            unimplemented!();
        }
    }
    
    
    
    //    Trait impl blocks allow associated constants, functions, types, and methods separated by normal semicolon syntax
    //    All of these must override or initialize the members from the initial "trait" block. No new ones can be defined.
    impl TraitExample for StructExample {
        //Associated Types (Must define type in def)
        //See Bottom
        
        //Associated Constant (Must define constant in def)
        const DEFINED_ASSOCIATED_CONSTANT: i32 = 1;
        
        //Associated Function (Must define function in def)
        fn defined_associated_function(radius: f64) -> Self::AbstractAssociatedType {
            unimplemented!();
        }
        
        //Method (Must define method in def)
        fn defined_method(&self) -> Self::AbstractAssociatedType {
            unimplemented!();
        }
        
        //Implement all abstractions from def
        type AbstractAssociatedType = i32;
        const ABSTRACT_ASSOCIATED_CONSTANT: i32 = 1;
        fn abstract_associated_function(){}
        fn abstract_method(&self){}
    }
}