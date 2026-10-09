fn main() {
    enum Enum {
        BasicExample,
        TupleExample(i32,f32),
    }
//    Enums can be straight up, or can represent named tuples

    enum Enum {
        BasicExample,
        TupleExample(i32,f32),
    }
    let e = Enum::TupleExample(32, 0.32);
    match e {
        Enum::BasicExample => println!("basicexample: {e:?}"),
        Enum::TupleExample(32,b) => println!("tupleexample1: {},{}",a,b),
        Enum::TupleExample(a,0.32) => println!("tupleexample2: {},{}",a,b),
        a => println!("default: {}", a),
    }
//    Enums can be partialy deconstructed in match statements like so
    
    
    enum Species {
        Crab,
        Octopus,
        Fish,
        Clam
    }
    struct SeaCreature {
        species: Species,
        name: String,
        arms: i32,
        legs: i32,
        weapon: String,
    }
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };
//    Enums are accessed via "NAMESPACE::" which is gross but whatever
}