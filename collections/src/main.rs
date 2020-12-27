// use std::ops::Add;

enum Form {
    Username(String),
    Age(i32),
    Status(bool)
}

fn main() {
    let _vector: Vec<i32> = Vec::new();

    {
        let mut nums: Vec<i32> = vec![1, 2, 3];
        nums.push(4);

        let fourth_num: &i32 = &nums[3];
        println!("Fourth Num: {}", fourth_num);

        match nums.get(3) {
            Some(fourth_num) => println!("Fourth Num: {}", fourth_num),
            None => println!("There is no fourth element")
        }

        {
            for num in &nums {
                println!("Num: {}", num);
            }
        }

        {
            for num in &mut nums {
                *num += 1;
                println!("Num: {}", num);
            }
        }

        println!("Second Num: {}", nums[1]);
    }

    {
        let _form_elements: Vec<Form> = vec![
            Form::Username(String::from("Express Gradient")),
            Form::Status(true)
        ];
    }

    {
        let mut some_string: String = String::new();
        let data: &str = "Hello World";
        some_string = data.to_string();

        some_string.push_str(" Again");
        some_string.push('!');
        println!("{}", some_string);

        let string_one: String = String::from("Hello");
        let string_two: String = String::from(" User");
        let greeting: String = string_one + &string_two;
        // let greeting: String = string_one.add(&string_two);
        println!("{}", greeting);
        let string_one: String = String::from("Hello");
        let _another_greeting = format!("{}-{}-{}", string_one, string_two, greeting);
    }

    {
        let some_string: &str = "String Slice";
        let first_char: &str = &some_string[0..1];
        println!("{}", first_char);
    }

    {
        let message: &str = "Welcome Home!";

        for message_char in message.chars() {
            println!("{}", message_char);
        }

        for message_byte in message.bytes() {
            println!("{}", message_byte);
        }
    }
}
