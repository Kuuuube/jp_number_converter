mod hiragana_tests;
mod kanji_tests;

mod hiragana;
mod kanji;

fn main() {
    loop {
        println!("Input: ");
        let input_string = read_line().trim().to_string();
        println!("{}", hiragana::convert_number(&input_string));
        println!("{}", kanji::convert_number(&input_string));
    }
}



fn read_line() -> String {
    let mut buffer = String::default();
    std::io::stdin().read_line(&mut buffer).unwrap_or_default();
    return buffer;
}

