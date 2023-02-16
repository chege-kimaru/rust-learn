fn main() {
    let list = vec![20, 30, 89, 45, 32];
    let res = largest_num(&list);
    println!("Largest 1 is: {}", res);

    // let list = vec![1, 90, 56, 89, 23, 1000, 23];
    let list = [1, 90, 56, 89, 23, 1000, 23];
    let res = largest_num(&list);
    println!("Largest 2 is: {}", res);


    let list_nums = vec![89, 100, 45, 67];
    let res = largest(&list_nums);
    println!("Largest number is: {}", res);
    let list_chars = vec!['e', 'a', 'd', 'b'];
    let res = largest(&list_chars);
    println!("Largest char is: {}", res);

    let both_integer = Point {x: 10, y: 20};
    let integer_and_float = Point2 {x: 5, y: 6.0};
    let both_float = Point2 {x: 5.0, y: 6.0};

    let p = Point {x: 10, y: 20};
    println!("p.x = {}", p.x());

    let p = Point {x: 10.0, y: 20.0};
    println!("distance_from_origin = {}", p.distance_from_origin());

    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest_num(list: &[i32]) -> &i32 {
    let mut largest_num = &list[0];

    for num in list {
        if num > largest_num {
            largest_num = num;
        }
    }

    largest_num
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

struct Point2<T, U> {
    x: T,
    y: U,
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y
        }
    }
}

enum Option<T> {
    Some(T),
    None
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}