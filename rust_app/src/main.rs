use std::pin::Pin;

#[repr(C)]
struct Data {
    value: i32,
}

extern "C" {
    fn c_take(d: *mut Data);
    fn c_increment();
    fn c_return() -> *mut Data;
}

struct OwnedByRust {
    data: Pin<Box<Data>>,
}

struct OwnedByC {
    ptr: *mut Data,
}

impl OwnedByRust {
    fn new(value: i32) -> Self {
        Self {
            data: Pin::new(Box::new(Data { value })),
        }
    }

    fn give_to_c(self) -> OwnedByC {
        //let ptr = unsafe { self.data.as_ref().get_ref() as *const Data as *mut Data };
        // Convert Pin<Box<Data>> to raw pointer without dropping the Box
        let ptr = Box::into_raw(unsafe { Pin::into_inner_unchecked(self.data) });
        unsafe { c_take(ptr) };
        OwnedByC { ptr }
    }

    fn value(&self) -> i32 {
        self.data.value
    }
}

impl OwnedByC {
    fn increment(&self) {
        unsafe { c_increment() }
    }

    fn take_back(self) -> OwnedByRust {
        let ptr = unsafe { c_return() };
        assert!(!ptr.is_null());

        let boxed = unsafe { Box::from_raw(ptr) };

        OwnedByRust { data: Pin::new(boxed) }
    }
}

fn main() {
    /*
     * Initial state: owned by Rust
     *
     * - Allocated on the heap
     * - Pinned: address is stable
     * - Rust has exclusive access
     * - Safe Rust may read, mutate, and borrow
     */
    let rust_owned = OwnedByRust::new(10);

    /*
     * Ordinary safe Rust access.
     *
     * - `value()` takes `&self`
     * - No aliasing
     * - No `unsafe`
     */
    println!("Rust sees {}", rust_owned.value());

    /*
     * Ownership handoff to C.
     *
     * - Rust transfers exclusive access
     * - A raw pointer is given to C
     * - `OwnedByRust` is consumed
     * - Rust cannot access the data in this state
     */
    let c_owned = rust_owned.give_to_c();

    // All access now goes through the C API
    c_owned.increment();
    c_owned.increment();

    // println!("{}", rust_owned.value()); //does not compile — ownership was consumed

    /*
     * Ownership returns to Rust.
     *
     * - C relinquishes the pointer
     * - Rust regains exclusive ownership
     * - Safe Rust access is restored
     */
    let rust_owned = c_owned.take_back();

    println!("Rust sees {}", rust_owned.value());
}
