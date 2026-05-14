/// Actions that can be done in unsafe rust
/// 1. Dereference a raw pointer
/// 2. Call an unsafe action or function
/// 3. Access or modify a mutable static variable
/// 4. Implement an unsafe trait
/// 5. Access fiels of unions
///
/// ## NEW TYPES
/// - raw pointers
///   - *const T (Immutable raw pointer)
///   - *mut T (Mutable raw pointer)
///
/// Raw pointers can have both mutable and immutable pointers
/// or multiple mutable pointers to the same location
/// They aren't guaranteed to point to valid memory
/// They are allowed to be null
/// They do not implement any automatic cleanup
///
fn main() {
    let mut num = 5;

    let r1 = &raw const num;

    let r3 = &raw mut num;

    let address = 0x012345usize;

    let r = address as *const i32;

    println!("{:?}", r);

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r3 is: {}", *r3);
    }
}
