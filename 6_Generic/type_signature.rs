fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {  // 같은 타입의 매개 변수 두 개를 입력받아 해당 타입의 값 반환
  i + j
}

// std::ops::Add<Output = T> ==> 타입 T가 std::ops::Add를 반드시 구현해야 함을 의미. [러스트는 모든 타입에 덧셈 연산을 지원하지 않으므로]
