fn main() {
    let a:i32 = 10;
    let b:u16 = 100;

    if a<b.try_into().unwrap(){  // mismatched types; try_into()로 i32 값으로 바꾸어 Result 반환하게 함. (unwrap으로 성공 값 처리)
        println!("100이 10보다 크다.");
    }
}

/*OUTPUT
100이 10보다 크다.
 */
