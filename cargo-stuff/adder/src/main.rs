use add_one;

fn main() {
    let num: i32 = 5;
    let squared_num: i32 = add_one::compute_square(num);

    println!("Square of five: {}", squared_num);
}
