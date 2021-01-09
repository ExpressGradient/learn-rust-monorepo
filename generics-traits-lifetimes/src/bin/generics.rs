struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MultiPoint<T, U> {
    x: T,
    y: U
}

impl<T, U> MultiPoint<T, U> {
    fn mix<V, W>(self, other: MultiPoint<V, W>) -> MultiPoint<T, W> {
        MultiPoint { x: self.x, y: other.y }
    }
}

fn main() {
    {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        let largest_num: &i32 = largest(&nums);
        println!("Largest num in nums: {}", largest_num);

        let chars: Vec<char> = vec!['a', 'b', 'c'];
        let largest_char: &char = largest(&chars);
        println!("Largest char in chars: {}", largest_char);
    }

    {
        let _integer: Point<i32> = Point { x: 2, y: 3 };
        let float: Point<f32> = Point { x: 2.0, y: 3.0 };
        let _multi: MultiPoint<i32, f32> = MultiPoint { x: 2, y: 3.0 };

        println!("Distance from origin: {}", float.distance_from_origin());
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}