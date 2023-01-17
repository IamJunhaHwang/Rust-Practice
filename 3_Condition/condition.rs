fn main() {
    let condition = true;
    let number = if condition {  // 이렇게 값을 할당할 때 if를 이용할 수 있음.
        5
    } else {
        6
    };

    println!("Num: {}", number);
}
