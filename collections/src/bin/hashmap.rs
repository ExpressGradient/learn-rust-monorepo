use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("RCB"), 23);
    scores.insert(String::from("SRH"), 22);
    println!("{:?}", scores);

    let team_name: String = String::from("RCB");
    let team_score_option: Option<&i32> = scores.get(&team_name);
    if let Some(team_score) = team_score_option {
        println!("RCB Score: {}", team_score);
    } else {
        println!("No such team");
    }

    for (team, score) in &scores {
        println!("{} => {}", team, score);
    }

    scores.insert(String::from("SRH"), 23);
    scores.entry(String::from("CSK")).or_insert(23);

    {
        let text: &str = "hello world wonderful world";

        let mut map: HashMap<&str, i32> = HashMap::new();

        for word in text.split_whitespace() {
            let count: &mut i32 = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}