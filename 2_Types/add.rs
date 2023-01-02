fn main() {
    /*
    let을 통해 변수를 바인딩 할 수 있으며 기본적으로 immutable하다.
    
    mutable한 변수로 선언하려면 let mut a = 7; 과 같이 선언해주어야한다.
    */
    
    let a = 7;  // 컴파일러가 타입 추론

    let b:i32 = 70;  // 프로그래머가 직접 지정, i32:integer 32비트 u32:unsigned integer 32비트
    let c = 15i32;
    let d = 15_i32;

    let e = add(add(a,b), add(c, d));

    println!("a+b+c+d = {}", e);
}

fn add(i:i32, j:i32) ->i32{  // 함수 정의에 타입 선언은 반드시 필요
    i+j  // 별도의 return 문이 필요 없음.
}

/*OUTPUT
a+b+c+d = 107
 */
