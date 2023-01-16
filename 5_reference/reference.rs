fn main() {
    let a = 42;
    let r = &a;  // a의 참조

    let b = a + *r;  // 역참조는 *를 이용한다.

    println!("a + a = {}", b);
}

/*output
a + a = 84
*/
