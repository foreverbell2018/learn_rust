use std::slice;

fn main() {
    println!("以下为 不安全操作 部分！");
    let raw_p: *const u32 = &10;
    unsafe {
        assert!(*raw_p == 10);
    }

    let some_vector = vec![1, 2, 3, 4];
    let pointer = some_vector.as_ptr();
    let length = some_vector.len();
    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);
        assert_eq!(some_vector.as_slice(), my_slice);
    }
}