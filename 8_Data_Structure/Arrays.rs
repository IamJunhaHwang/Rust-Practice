fn main() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];  // [T; n] ==> 타입 T가 n개
    let blank1 = [0; 3];  // 0 이 3개 들어간다는 뜻
    let blank2: [u8; 3] = [0; 3];

    let arrays = [one, two, blank1, blank2];

    for a in &arrays{
        print!("{:?}: ", a);  // 배열 출력

        for n in a.iter(){
            println!("\t{} + 10 = {}", n , n+10);
        }

        let mut sum = 0;  // 변수 sum, let만 있으면 상수
        for i in 0..a.len(){
            sum += a[i];
        }
        println!("\t sum of {:?} = {}", a, sum);
    }
}
