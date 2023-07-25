use std::collections::HashMap;

pub fn convert_number(input_string: &str) -> String {
    match input_string.len() {
        1 => return digits_1(input_string, true),
        2 => return digits_2(input_string),
        3 => return digits_3(input_string),
        4 => return digits_4(input_string, false),
        5 => return digits_5(input_string),
        6 => return digits_6(input_string),
        7 => return digits_7(input_string),
        8 => return digits_8(input_string),
        9 => return digits_9(input_string),
        10 => return digits_10(input_string),
        11 => return digits_11(input_string),
        12 => return digits_12(input_string),
        13 => return digits_13(input_string),
        14 => return digits_14(input_string),
        15 => return digits_15(input_string),
        16 => return digits_16(input_string),
        _ => return "".to_string()
    }
}

fn digits_1(input_string: &str, use_zero: bool) -> String {
    let dict: HashMap<&str, &str> = HashMap::from([
        ("0", ""),
        ("1", "壱"),
        ("2", "弐"),
        ("3", "参"),
        ("4", "四"),
        ("5", "伍"),
        ("6", "六"),
        ("7", "七"),
        ("8", "八"),
        ("9", "九")
    ]);
    if input_string == "0" && use_zero {
        return "零".to_string();
    }
    return dict.get(input_string).unwrap_or(&"").to_string();
}

fn digits_2(input_string: &str) -> String {
    let dict: HashMap<&str, &str> = HashMap::from([
        ("0", ""),
        ("1", ""),
        ("2", "弐"),
        ("3", "参"),
        ("4", "四"),
        ("5", "伍"),
        ("6", "六"),
        ("7", "七"),
        ("8", "八"),
        ("9", "九")
    ]);
    let first_char = &input_string[0..1];
    let second_char = &input_string[1..];
    if first_char != "0" {
        return format!("{}{}{}", dict.get(first_char).unwrap().to_string(), "拾", digits_1(second_char, false));
    }
    return format!("{}{}", dict.get(first_char).unwrap().to_string(), digits_1(second_char, false));
}

fn digits_3(input_string: &str) -> String {
    let dict: HashMap<&str, &str> = HashMap::from([
        ("0", ""),
        ("1", ""),
        ("2", "弐"),
        ("3", "参"),
        ("4", "四"),
        ("5", "伍"),
        ("6", "六"),
        ("7", "七"),
        ("8", "八"),
        ("9", "九")
    ]);
    let first_char = &input_string[0..1];
    let remaining_chars = &input_string[1..];
    if first_char == "0" {
        return format!("{}{}", dict.get(first_char).unwrap().to_string(), digits_2(remaining_chars));
    }
    return format!("{}{}{}", dict.get(first_char).unwrap().to_string(), "佰", digits_2(remaining_chars))
}

fn digits_4(input_string: &str, prefix: bool) -> String {
    let dict: HashMap<&str, &str> = HashMap::from([
        ("0", ""),
        ("1", ""),
        ("2", "弐"),
        ("3", "参"),
        ("4", "四"),
        ("5", "伍"),
        ("6", "六"),
        ("7", "七"),
        ("8", "八"),
        ("9", "九")
    ]);
    let first_char = &input_string[0..1];
    let remaining_chars = &input_string[1..];
    if first_char == "0" {
        return format!("{}{}", dict.get(first_char).unwrap().to_string(), digits_3(remaining_chars));
    }
    if first_char == "1" && prefix {
        return format!("{}{}{}", dict.get(first_char).unwrap().to_string(), "壱阡", digits_3(remaining_chars))
    }
    return format!("{}{}{}", dict.get(first_char).unwrap().to_string(), "阡", digits_3(remaining_chars))
}

fn digits_5(input_string: &str) -> String {
    let dict: HashMap<&str, &str> = HashMap::from([
        ("0", ""),
        ("1", "壱"),
        ("2", "弐"),
        ("3", "参"),
        ("4", "四"),
        ("5", "伍"),
        ("6", "六"),
        ("7", "七"),
        ("8", "八"),
        ("9", "九")
    ]);
    let first_char = &input_string[0..1];
    let remaining_chars = &input_string[1..];
    return format!("{}{}{}", dict.get(first_char).unwrap().to_string(), "萬", digits_4(remaining_chars, true));
}

fn digits_6(input_string: &str) -> String {
    let first_chars = &input_string[0..2];
    let remaining_chars = &input_string[2..];
    return format!("{}{}{}", digits_2(first_chars), "萬", digits_4(remaining_chars, true));
}

fn digits_7(input_string: &str) -> String {
    let first_chars = &input_string[0..3];
    let remaining_chars = &input_string[3..];
    return format!("{}{}{}", digits_3(first_chars), "萬", digits_4(remaining_chars, true));
}

fn digits_8(input_string: &str) -> String {
    let first_chars = &input_string[0..4];
    let remaining_chars = &input_string[4..];
    return format!("{}{}{}", digits_4(first_chars, true), "萬", digits_4(remaining_chars, true));
}

fn digits_9(input_string: &str) -> String {
    let dict: HashMap<&str, &str> = HashMap::from([
        ("0", ""),
        ("1", "壱"),
        ("2", "弐"),
        ("3", "参"),
        ("4", "四"),
        ("5", "伍"),
        ("6", "六"),
        ("7", "七"),
        ("8", "八"),
        ("9", "九")
    ]);
    let first_char = &input_string[0..1];
    let remaining_chars_0 = &input_string[1..5];
    let remaining_chars_1 = &input_string[5..];
    if remaining_chars_0 != "0000" {
        return format!("{}{}{}{}{}", dict.get(first_char).unwrap().to_string(), "億", digits_4(remaining_chars_0, true), "萬", digits_4(remaining_chars_1, true));
    }
    return format!("{}{}{}{}", dict.get(first_char).unwrap().to_string(), "億", digits_4(remaining_chars_0, true), digits_4(remaining_chars_1, true));
}

fn digits_10(input_string: &str) -> String {
    let first_chars = &input_string[0..2];
    let remaining_chars_0 = &input_string[2..6];
    let remaining_chars_1 = &input_string[6..];
    if remaining_chars_0 != "0000" {
        return format!("{}{}{}{}{}", digits_2(first_chars), "億", digits_4(remaining_chars_0, true), "萬", digits_4(remaining_chars_1, true));
    }
    return format!("{}{}{}{}", digits_2(first_chars), "億", digits_4(remaining_chars_0, true), digits_4(remaining_chars_1, true));
}

fn digits_11(input_string: &str) -> String {
    let first_chars = &input_string[0..3];
    let remaining_chars_0 = &input_string[3..7];
    let remaining_chars_1 = &input_string[7..];
    if remaining_chars_0 != "0000" {
        return format!("{}{}{}{}{}", digits_3(first_chars), "億", digits_4(remaining_chars_0, true), "萬", digits_4(remaining_chars_1, true));
    }
    return format!("{}{}{}{}", digits_3(first_chars), "億", digits_4(remaining_chars_0, true), digits_4(remaining_chars_1, true));
}

fn digits_12(input_string: &str) -> String {
    let first_chars = &input_string[0..4];
    let remaining_chars_0 = &input_string[4..8];
    let remaining_chars_1 = &input_string[8..];
    if remaining_chars_0 != "0000" {
        return format!("{}{}{}{}{}", digits_4(first_chars, true), "億", digits_4(remaining_chars_0, true), "萬", digits_4(remaining_chars_1, true));
    }
    return format!("{}{}{}{}", digits_4(first_chars, true), "億", digits_4(remaining_chars_0, true), digits_4(remaining_chars_1, true));
}

fn digits_13(input_string: &str) -> String {
    let dict: HashMap<&str, &str> = HashMap::from([
        ("0", ""),
        ("1", "壱"),
        ("2", "弐"),
        ("3", "参"),
        ("4", "四"),
        ("5", "伍"),
        ("6", "六"),
        ("7", "七"),
        ("8", "八"),
        ("9", "九")
    ]);
    let first_char = &input_string[0..1];
    let remaining_chars_0 = &input_string[1..5];
    let remaining_chars_1 = &input_string[5..9];
    let remaining_chars_2 = &input_string[9..];
    if remaining_chars_0 != "0000" {
        return format!("{}{}{}{}", dict.get(first_char).unwrap().to_string(), "兆", digits_4(remaining_chars_0, true), digits_9(&format!("{}{}{}", "0", remaining_chars_1.to_string(), remaining_chars_2)));
    }
    if remaining_chars_1 != "0000" {
        return format!("{}{}{}{}", dict.get(first_char).unwrap().to_string(), "兆", digits_4(remaining_chars_0, true), digits_8(&format!("{}{}", remaining_chars_1.to_string(), remaining_chars_2)));
    }
    return format!("{}{}{}{}{}", dict.get(first_char).unwrap().to_string(), "兆", digits_4(remaining_chars_0, true), digits_4(remaining_chars_1, true), digits_4(remaining_chars_2, true));
}

fn digits_14(input_string: &str) -> String {
    let first_chars = &input_string[0..2];
    let remaining_chars_0 = &input_string[2..6];
    let remaining_chars_1 = &input_string[6..10];
    let remaining_chars_2 = &input_string[10..];

    if remaining_chars_0 != "0000" {
        return format!("{}{}{}{}", digits_2(first_chars), "兆", digits_4(remaining_chars_0, true), digits_9(&format!("{}{}{}", "0", remaining_chars_1.to_string(), remaining_chars_2)));
    }
    if remaining_chars_1 != "0000" {
        return format!("{}{}{}{}", digits_2(first_chars), "兆", digits_4(remaining_chars_0, true), digits_8(&format!("{}{}", remaining_chars_1.to_string(), remaining_chars_2)));
    }
    return format!("{}{}{}{}{}", digits_2(first_chars), "兆", digits_4(remaining_chars_0, true), digits_4(remaining_chars_1, true), digits_4(remaining_chars_2, true));
}

fn digits_15(input_string: &str) -> String {
    let first_chars = &input_string[0..3];
    let remaining_chars_0 = &input_string[3..7];
    let remaining_chars_1 = &input_string[7..11];
    let remaining_chars_2 = &input_string[11..];
    if remaining_chars_0 != "0000" {
        return format!("{}{}{}{}", digits_3(first_chars), "兆", digits_4(remaining_chars_0, true), digits_9(&format!("{}{}{}", "0", remaining_chars_1.to_string(), remaining_chars_2)));
    }
    if remaining_chars_1 != "0000" {
        return format!("{}{}{}{}", digits_3(first_chars), "兆", digits_4(remaining_chars_0, true), digits_8(&format!("{}{}", remaining_chars_1.to_string(), remaining_chars_2)));
    }
    return format!("{}{}{}{}{}", digits_3(first_chars), "兆", digits_4(remaining_chars_0, true), digits_4(remaining_chars_1, true), digits_4(remaining_chars_2, true));
}

fn digits_16(input_string: &str) -> String {
    let first_chars = &input_string[0..4];
    let remaining_chars_0 = &input_string[4..8];
    let remaining_chars_1 = &input_string[8..12];
    let remaining_chars_2 = &input_string[12..];
    if remaining_chars_0 != "0000" {
        return format!("{}{}{}{}", digits_4(first_chars, true), "兆", digits_4(remaining_chars_0, true), digits_9(&format!("{}{}{}", "0", remaining_chars_1.to_string(), remaining_chars_2)));
    }
    return format!("{}{}{}{}{}", digits_4(first_chars, true), "兆", digits_4(remaining_chars_0, true), digits_4(remaining_chars_1, true), digits_4(remaining_chars_2, true));
}