struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

fn main() {
    let pointer_one: CustomSmartPointer = CustomSmartPointer { data: String::from("Hello") };
    let pointer_two: CustomSmartPointer = CustomSmartPointer { data: String::from("World") };

    println!("CustomSmartPointers pointer_one and pointer_two created");
    drop(pointer_one);
    println!("CustomSmartPointer pointer_one is dropped before the end of its scope");
}