fn main() {
    let mut a = [10, 20, 30, 40, 50];

    for element in a.iter() {  // &i32
        println!("the value is: {}", element);
    }

    for element in a {  // i32
        println!("the value is: {}", element);
    }

    for element in a.iter_mut() {  // mutable
        println!("the value is: {}", element);
    }
}
