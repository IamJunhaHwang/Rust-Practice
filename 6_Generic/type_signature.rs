use std::ops::{Add};
use std::time::{Duration};

fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {  // 같은 타입의 매개 변수 두 개를 입력받아 해당 타입의 값 반환
    i + j
}

// std::ops::Add<Output = T> ==> 타입 T가 std::ops::Add를 반드시 구현해야 함을 의미. [러스트는 모든 타입에 덧셈 연산을 지원하지 않으므로]

fn main() {
    let floats = add(1.2, 3.4);
    let ints = add(10,20);
    let durations = add(
        Duration::new(5,0),
        Duration::new(10,0)
    );

    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations);  // duration은 std::fmt::Disply 트레이트를 구현하지 않으므로 std::fmt::Debug를 씀.
}
