//    Arrays are predefined segment of memory, often on the stack
//    Vectors are growable segment of memory, often in the heap

//    Vectors have a pointer to the data, a length of actually added data, and a capacity of reserved memory for data
//    Upon surpassing capacity, more memory is reserved often going from capacity n -> 2n
//      The entire new capacity is reserved elsewhere, the existing block is copied into the new block, and the old block is freed

//    Vectors are initialized with Vec::new() or Vec::with_capacity(n) or vec![]
let mut vec_no_capacity: Vec<i32> = Vec::new(); // Initialize with 0 length and capacity
let mut vec_with_capacity: Vec<i32> = Vec::with_capacity(5); // Initialize with 0 length and 5 capacity
let vec = vec![1, 2, 3]; // Shorthand syntax initializing with values
//    Arrays are initialized with []
let arr1 = [1, 2, 3, 4, 5]; // Initialize with direct values
let arr2 = [0; 10]; // Initialize with repeated values
let arr3: [i32; 5] = [0;5] // Explicit type annotation
let arr4: [i32; 5] = array::from_fn(|i| (i + 1) * 100); // From function usage - Takes named function or closure which accepts one argument of type usize that represents index
println!("{:?}", array::from_fn::<i32, 5, _>(generate_value)); // Advanced from function usage - generics are manually entered rather than implied