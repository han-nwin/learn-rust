fn main() {
    //NOTE: Dereferencing a Raw Pointer
    // Are allowed to ignore the borrowing rules by having both immutable and
    // mutable pointers or multiple mutable pointers to the same location
    // Aren’t guaranteed to point to valid memory
    // Are allowed to be null
    // Don’t implement any automatic cleanup

    let mut num = 5;

    let r1 = &raw const num; // creates a *const i32 immutable raw pointer
    let r2 = &raw mut num; // creates a *mut i32 mutable raw pointer
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    // create a raw pointer from an arbitrary address
    let address = 0x012345usize;
    let r = address as *const i32;

    // Calling an unsafe function
    unsafe {
        dangerous();
    }

    // Creating safe abstraction for unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3); //NOTE: Study this function

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        use std::slice;
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        // Compiler don't know these 2 are non overlapping mutatble reference
        // (&mut values[..mid], &mut values[mid..])

        // Use unsafe code instead
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
}

unsafe fn dangerous() {}
