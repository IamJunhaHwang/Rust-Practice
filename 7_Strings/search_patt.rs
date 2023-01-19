fn main() {
    let search_term = "태극기";
    let quote = "\
나는 자랑스러운 태극기 앞에
자유롭고 정의로운 대한민국의 무궁한 영광을 위하여
충성을 다할 것을 굳게 다짐합니다.";

    for line in quote.lines() {                    // 텍스트 한 줄씩 받아오기
        if line.contains(search_term) {
            println!("Find! ==> {}", line);
        }
        else{
            println!("{}", line);
        }
    }
}
