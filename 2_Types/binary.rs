fn main() {
    let three = 0b11;  // b = binary
    let thirty = 0o36;  // o = octal
    let three_hundred = 0x12C;  // x = hexadecimal

    println!("base 10= {}, {}, {}", three, thirty, three_hundred);
    println!("base 2= {:b}, {:b}, {:b}", three, thirty, three_hundred);
    println!("base 8= {:o}, {:o}, {:o}", three, thirty, three_hundred);
    println!("base 16= {:x}, {:x}, {:x}", three, thirty, three_hundred);
}

/*OUTPUT
base 10= 3, 30, 300
base 2= 11, 11110, 100101100
base 8= 3, 36, 454
base 16= 3, 1e, 12c
 */
