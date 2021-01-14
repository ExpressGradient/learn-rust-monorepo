struct User<'a> {
    username: &'a str
}

fn main() {
    // {
    //     let r;                  // + 'a
    //                             // |
    //     {                       // |
    //         let x = 5;          // + 'b
    //         r = &x;             // |
    //     }                       // + 'b
    //                             // |
    //     println!("r: {}", r);   // |
    // }                           // + 'a

    {
        let x = 5;              // + 'b
                                // |
        let r = &x;             // + 'a
                                // |
        println!("r: {}", r);   // |
    }                           // + 'a 'b

    {
        let str1: String = String::from("abcd");
        let str2: &str = "xyz";
        let result: &str = longest(str1.as_str(), str2);
        println!("The longest string is {}", result);
    }

    {
        let user: User = User {
            username: "discoding"
        };
        println!("Username: {}", user.username);
    }

    let _string: &'static str = "I have a static lifetime";
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}