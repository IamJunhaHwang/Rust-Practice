use std::time::{Duration, Instant};  // std::time에서 Duration과 Instant만 가져옴.

fn main() {
    let mut count = 0;  // mutable 변수 count 선언
    let time_limit = Duration::new(1,0);  // 1초를 나타내는 Duration 생성
    let start = Instant::now();  // 시스템 내장 시계 값 읽어오기

    while(Instant::now() - start) < time_limit{  // Instant - Instant = Duration
        count +=1;
    }

    println!("{}", count);
}
