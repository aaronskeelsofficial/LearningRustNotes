#[derive(Default,Debug)]
struct Structure1 {
    value1: i32,
    value2: i32,
}

//----------------------

#[derive(Debug)]
struct Structure2 {
    value1: f32,
    value2: f32,
}
impl Structure2 {
    fn default() -> Self {
        Structure2 {
            value1: 0.1f32,
            value2: 0.1f32,
        }
    }
}
impl Default for Structure2 {
    fn default() -> Self {
        Structure2 {
            value1: 0.0,
            value2: 0.0,
        }
    }
}

//----------------------

fn main() {
    let v = Structure1 {value1: 1, ..Default::default() }; //(1,0)
    dbg!(v);
    let v = Structure1 {value1: 1, ..Structure1::default() }; //(1,0)
    dbg!(v);
    let v = Structure1 {value1: 1, ..<Structure1 as Default>::default() }; //(1,0)
    dbg!(v);
    let v: Structure2 = Default::default(); //(0,0)
    dbg!(v);
    let v: Structure2 = Structure2::default(); //(0.1,0.1)
    dbg!(v);
    let v: Structure2 = <Structure2 as Default>::default(); //(0,0)
    dbg!(v);
//    let v: Structure2 = <Default as Structure2>::default(); //Error: "as" only applies as trait, not as struct
//    dbg!(v);
}