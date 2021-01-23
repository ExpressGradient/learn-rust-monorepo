struct ClosureStruct<T> where T: Fn(Vec<u32>) -> bool {
    closure: T,
}

fn main() {
    let x: Vec<u32> = vec![1, 2, 3];

    let equal_to_x = |z| z == x;
    let _closure_struct = ClosureStruct {
        closure: equal_to_x
    };

    let y: Vec<u32> = vec![1, 2, 3];
    assert!(equal_to_x(y));
}