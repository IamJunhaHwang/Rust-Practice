/*
dependencies: num = "0.4"
*/

use num::complex::Complex;                      

fn main() {
  let a = Complex { re: 2.1, im: -1.2 };  // 생성자가 없는 대신 모든 타입에 리터럴 형태가 있다.
  let b = Complex::new(11.1, 22.2);  // 타입은 정적 메소드인 new를 사용할 수 있지만 인스턴스는 사용할 수 없다.
  let result = a + b;

  println!("{} + {}i", result.re, result.im)  // .을 이용해 접근   
}

/*OUTPUT
2.1 + -1.2i
*/
