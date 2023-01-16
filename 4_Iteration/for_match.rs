

fn main() {
    let needle = 42;

    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",  // 42나 132면 result에 hit! 할당
            10..=20 => "test",   // 10..20 result에 test 할당
            _ => "miss",         // 그 외에는 miss 할당
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
        else if result == "test" {
            println!("{}: {}", item, result);
        }
        else if result == "miss" {
            println!("{}: {}", item, result);
        }
    }
}
