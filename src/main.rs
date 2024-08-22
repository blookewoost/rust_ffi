pub mod bindings;

fn main() {
    // Perform a mathematical operation through the C FFI to libm
    let x: f64 = 4.0;
    let result: f64;

    unsafe {
        result = bindings::sqrt(x);
    }

    assert!(result == 2.0);
    
}
