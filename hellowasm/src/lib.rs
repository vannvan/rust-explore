#[no_mangle]
pub extern "C" fn fbin(x: i32) -> i32 {
    if x <= 1 {
        return 1;
    } else {
        return fbin(x - 1) + fbin(x - 2);
    }
}
