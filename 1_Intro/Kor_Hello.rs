fn main() {
    println!("Hello, world!");
    let korean = "\
    안녕,
    세상아!
    ";

    let ko_hello = [korean];

    let tmp = korean.lines();  // 줄 단위로 끊기

    for sen in ko_hello.iter(){  // iteration으로 가져올 수 있음.
        println!("{}", &sen);
    }

    for (idx, line) in tmp.enumerate(){  // 파이썬의 enumerate와 같이 idx와 변수 함께 출력 ()로 감싸주어야 함.
        println!("{}", idx);
        println!("___{}___ ___{}___", line, line.trim())  // trim을 통해 양 옆 공백을 없앨 수 있음.
    }
}
