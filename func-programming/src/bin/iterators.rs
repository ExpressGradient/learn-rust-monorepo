#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == shoe_size).collect()
}

fn main() {
    let vector: Vec<u32> = vec![1, 2, 3];
    let mut vector_iter = vector.iter();

    println!("Num1: {:?}", vector_iter.next());

    for num in vector_iter {
        println!("Num: {}", num);
    }

    let nums: Vec<u32> = vec![1, 2, 3];
    let total: u32 = nums.iter().sum();

    println!("Nums total: {}", total);

    let nums: Vec<u32> = vec![1, 2, 3];
    let incr_nums: Vec<u32> = nums.iter().map(|x| x + 1).collect();

    println!("Incremented nums: {:?}", incr_nums);
}

#[cfg(test)]
mod tests {
    use super::{Shoe, shoes_in_my_size};

    #[test]
    fn test_shoes_in_my_size() {
        let shoes: Vec<Shoe> = vec![
            Shoe {
                size: 10,
                style: String::from("Loafer")
            },
            Shoe {
                size: 10,
                style: String::from("Formal")
            },
            Shoe {
                size: 11,
                style: String::from("Formal")
            }
        ];

        let filtered_shoes: Vec<Shoe> = shoes_in_my_size(shoes, 11);
        let correct_shoes: Vec<Shoe> = vec![
            Shoe {
                size: 11,
                style: String::from("Formal")
            }
        ];

        assert_eq!(filtered_shoes, correct_shoes);
    }
}