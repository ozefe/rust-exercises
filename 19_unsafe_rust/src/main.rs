use std::slice;

static _HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

unsafe trait _Foo {}

unsafe impl _Foo for i32 {}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address: u32 = 0xDEADBEEF;
    let _r = address as *const u32;

    add_to_count(3);

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
        // println!("r is {}", *_r);

        dangerous();

        println!("Absolute value of -3 according to C: {}", abs(-3));

        println!("COUNTER: {COUNTER}");
    }

    let mut v = [1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    assert!(mid <= len);

    let ptr = values.as_mut_ptr();

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
