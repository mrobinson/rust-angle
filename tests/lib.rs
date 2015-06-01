extern crate angle;

use angle::glslang::*;

#[test]
fn basic_shader() {
    unsafe {
        assert!(Initialize());
    }
}
