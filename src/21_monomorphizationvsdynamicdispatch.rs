/*
 * Monomorphization (Static Dispatch)
 * The compiler generates a distinct, specialized version of the machine code for every type factually
 *  used in the code being compiled (NOT for all POTENTIAL types).
 * This is NOT runtime method lookup, virtual functions, nor object-oriented class inheritance.
 */
trait Speak { fn speak(&self); }
// Generic: Monomorphized per type
fn announce<T: Speak>(item: T) {
    item.speak();
}
// Behind the scenes, Rust generates:
// fn announce_dog(item: Dog) { item.speak(); }
// fn announce_cat(item: Cat) { item.speak(); }

/*
 * Dynamic Dispatch (dyn Trait)
 * Runtime vtable indirection. A single compiled function receives a fat pointer (data pointer + vtable pointer)
 *  and resolves method calls during execution.
 * This is NOT zero-cost inlining or compile-time specialization.
 */
trait Speak { fn speak(&self); }
// Trait Object: Single function compiled
fn announce(item: &dyn Speak) {
    item.speak(); // Looks up speak() address in item's vtable at runtime
}

/*
 * Comparisons:
 *  Dispatch Timing:
 *      Mono: Compile time
 *      Gen: Runtime (vtable lookup)
 *  CPU Execution:
 *      Mono: Fastest (direct jump/inlined)
 *      Gen: Slower (indirect pointer branch)
 *  Binary Executable Size:
 *      Mono: Larger (code duplicated per type)
 *      Gen: Smaller (single shared implementation)
 *  Data Memory Layout:
 *      Mono: Compact (contiguous stack/heap values)
 *      Gen: Padded (requires 16-byte fat pointer redirects to objects spread across memory)
 *  Collections:
 *      Mono: Homogeneous (Vec<Dog> only)
 *      Gen: Heterogeneous (Vec<Box<dyn Speak>> allows elements of Dog or Cat)
 */

/* Homogenous Layout (Vec<Dog>)
STACK                             HEAP (Single Contiguous Buffer)
+-----------------------+         +---------------+---------------+---------------+
| Vec<Dog>              |         | Dog #0        | Dog #1        | Dog #2        |
|  - ptr -----------------------> | age: 3        | age: 5        | age: 1        |
|  - cap: 4             |         | id: 101       | id: 102       | id: 103       |
|  - len: 3             |         +---------------+---------------+---------------+
+-----------------------+         0x1000          0x1008          0x1010
 */
/* Heterogeneous Layout (Vec<Box<Dog>>)
STACK                             HEAP: Vec Buffer (Contiguous 16-Byte Fat Pointers)
+-----------------------+         +-------------------------------------------------------------+
| Vec<Box<dyn Speak>>   |         | Slot 0: [ data_ptr: 0x5000 | vtable_ptr: 0x8000 (Dog) ]    |
|  - ptr -----------------------> | Slot 1: [ data_ptr: 0x6000 | vtable_ptr: 0x8020 (Cat) ]    |
|  - cap: 4             |         | Slot 2: [ data_ptr: 0x7000 | vtable_ptr: 0x8000 (Dog) ]    |
|  - len: 3             |         +-------------------------------------------------------------+
+-----------------------+                    |                  |
                                             |                  +--------------------+
                                             v                                       v
                               HEAP: Individual Allocations               .RODATA: Static Vtables
                               +-----------------------+                  +-----------------------+
                        0x5000 | Dog { age: 3, id:101 }|           0x8000 | Dog_VTable            |
                               +-----------------------+                  |  speak -> 0x401000    |
                        0x6000 | Cat { lives: 9 }      |                  +-----------------------+
                               +-----------------------+           0x8020 | Cat_VTable            |
                        0x7000 | Dog { age: 1, id:103 }|                  |  speak -> 0x402000    |
                               +-----------------------+                  +-----------------------+
 */