use std::collections::HashMap;

pub fn convert_number(input_string: &str) -> String {
    let split_string: Vec<&str> = input_string.split(".").collect();
    if split_string.len() > 1 {
        return format!("{}点{}", whole_number(split_string.get(0).unwrap(), true), fraction(split_string.get(1).unwrap()));
    }
    return whole_number(input_string, true);
}

fn whole_number(input_string: &str, init: bool) -> String {
    match input_string.len() {
        1 => return digits_1(input_string, init),
        2 => return digits_2(input_string),
        3 => return digits_3(input_string),
        4 => return digits_4(input_string, !init),
        5..=8 => return digits_5_8(input_string),
        9..=12 => return digits_9_12(input_string),
        13..=16 => return digits_13_16(input_string),
        17..=20 => return digits_17_20(input_string),
        21..=24 => return digits_21_24(input_string),
        25..=28 => return digits_25_28(input_string),
        29..=32 => return digits_29_32(input_string),
        33..=36 => return digits_33_36(input_string),
        37..=40 => return digits_37_40(input_string),
        41..=44 => return digits_41_44(input_string),
        45..=48 => return digits_45_48(input_string),
        49..=52 => return digits_49_52(input_string),
        53..=56 => return digits_53_56(input_string),
        57..=60 => return digits_57_60(input_string),
        61..=64 => return digits_61_64(input_string),
        65..=68 => return digits_65_68(input_string),
        69..=72 => return digits_69_72(input_string),
        _ => return "".to_string()
    }
}

fn fraction(input_string: &str) -> String {
    return input_string.replace("0", "零").replace("1", "一").replace("2", "二").replace("3", "三").replace("4", "四").replace("5", "五").replace("6", "六").replace("7", "七").replace("8", "八").replace("9", "九").to_string();
}

fn dict_get(input_string: &str, use_one: bool) -> String {
    let dict: HashMap<&str, &str> = HashMap::from([
        ("1", "一"),
        ("2", "二"),
        ("3", "三"),
        ("4", "四"),
        ("5", "五"),
        ("6", "六"),
        ("7", "七"),
        ("8", "八"),
        ("9", "九")
    ]);
    if input_string == "1" && !use_one {
        return "".to_string();
    }
    return dict.get(input_string).unwrap_or(&"").to_string();
}

fn handle_first_chars(first_chars: &str) -> String {
    match first_chars.len() {
        1 => return dict_get(first_chars, true),
        2 => return digits_2(first_chars),
        3 => return digits_3(first_chars),
        4 => return digits_4(first_chars, true),
        _ => return "".to_string()
    }
}

fn trim_and_recurse(input_string: &str) -> String {
    return whole_number(input_string.trim_start_matches("0"), false);
}

fn digits_1(input_string: &str, use_zero: bool) -> String {
    if input_string == "0" && use_zero {
        return "零".to_string();
    }
    return dict_get(input_string, true);
}

fn digits_2(input_string: &str) -> String {
    let first_char = &input_string[0..1];
    let second_char = &input_string[1..];
    if first_char != "0" {
        return format!("{}{}{}", dict_get(first_char, false), "十", digits_1(second_char, false));
    }
    return format!("{}{}", dict_get(first_char, false), digits_1(second_char, false));
}

fn digits_3(input_string: &str) -> String {
    let first_char = &input_string[0..1];
    let remaining_chars = &input_string[1..];
    match first_char {
        "0" => return format!("{}{}", dict_get(first_char, false), digits_2(remaining_chars)),
        _ => return format!("{}{}{}", dict_get(first_char, false), "百", digits_2(remaining_chars))
    }
}

fn digits_4(input_string: &str, prefix: bool) -> String {
    let first_char = &input_string[0..1];
    let remaining_chars = &input_string[1..];
    if first_char == "0" {
        return format!("{}{}", dict_get(first_char, false), digits_3(remaining_chars));
    }
    if first_char == "1" && prefix {
        return format!("{}{}{}", dict_get(first_char, false), "一千", digits_3(remaining_chars))
    }
    return format!("{}{}{}", dict_get(first_char, false), "千", digits_3(remaining_chars))
}

fn digits_5_8(input_string: &str) -> String {
    let digits = input_string.len();
    let first_chars = &input_string[0..digits - 4];
    let remaining_chars_0 = &input_string[digits - 4..];

    return format!("{}{}{}", handle_first_chars(first_chars), "万", digits_4(remaining_chars_0, true));
}

fn digits_9_12(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 12;
    let digit_range_characters = "億";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}{}", handle_first_chars(&input_string[0..digits - (max_len - 4)]), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), "万", digits_4(&input_string[digits - (max_len - 8)..digits - (max_len - 12)], true));
    }
    return format!("{}{}{}", handle_first_chars(&input_string[0..digits - (max_len - 4)]), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 12)]));
}

fn digits_13_16(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 16;
    let first_chars = &input_string[0..digits - (max_len - 4)];
    let digit_range_characters = "兆";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}", handle_first_chars(first_chars), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), digits_9_12(&format!("{}{}", "0", &input_string[digits - (max_len - 8)..])));
    }
    return format!("{}{}{}", handle_first_chars(first_chars), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 16)]));
}

fn digits_17_20(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 20;
    let first_chars = &input_string[0..digits - (max_len - 4)];
    let digit_range_characters = "京";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}", handle_first_chars(first_chars), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), digits_13_16(&format!("{}{}", "0", &input_string[digits - (max_len - 8)..])));
    }
    return format!("{}{}{}", handle_first_chars(first_chars), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 20)]));
}

fn digits_21_24(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 24;
    let first_chars = &input_string[0..digits - (max_len - 4)];
    let digit_range_characters = "垓";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}", handle_first_chars(first_chars), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), digits_17_20(&format!("{}{}", "0", &input_string[digits - (max_len - 8)..])));
    }
    return format!("{}{}{}", handle_first_chars(first_chars), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 24)]));
}

fn digits_25_28(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 28;
    let first_chars = &input_string[0..digits - (max_len - 4)];
    let digit_range_characters = "𥝱";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}", handle_first_chars(first_chars), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), digits_21_24(&format!("{}{}", "0", &input_string[digits - (max_len - 8)..])));
    }
    return format!("{}{}{}", handle_first_chars(first_chars), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 28)]));
}

fn digits_29_32(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 32;
    let first_chars = &input_string[0..digits - (max_len - 4)];
    let digit_range_characters = "穣";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}", handle_first_chars(first_chars), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), digits_25_28(&format!("{}{}", "0", &input_string[digits - (max_len - 8)..])));
    }
    return format!("{}{}{}", handle_first_chars(first_chars), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 32)]));
}

fn digits_33_36(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 36;
    let first_chars = &input_string[0..digits - (max_len - 4)];
    let digit_range_characters = "溝";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}", handle_first_chars(first_chars), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), digits_29_32(&format!("{}{}", "0", &input_string[digits - (max_len - 8)..])));
    }
    return format!("{}{}{}", handle_first_chars(first_chars), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 36)]));
}

fn digits_37_40(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 40;
    let first_chars = &input_string[0..digits - (max_len - 4)];
    let digit_range_characters = "澗";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}", handle_first_chars(first_chars), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), digits_33_36(&format!("{}{}", "0", &input_string[digits - (max_len - 8)..])));
    }
    return format!("{}{}{}", handle_first_chars(first_chars), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 40)]));
}

fn digits_41_44(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 44;
    let first_chars = &input_string[0..digits - (max_len - 4)];
    let digit_range_characters = "正";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}", handle_first_chars(first_chars), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), digits_37_40(&format!("{}{}", "0", &input_string[digits - (max_len - 8)..])));
    }
    return format!("{}{}{}", handle_first_chars(first_chars), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 44)]));
}

fn digits_45_48(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 48;
    let first_chars = &input_string[0..digits - (max_len - 4)];
    let digit_range_characters = "載";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}", handle_first_chars(first_chars), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), digits_41_44(&format!("{}{}", "0", &input_string[digits - (max_len - 8)..])));
    }
    return format!("{}{}{}", handle_first_chars(first_chars), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 48)]));
}

fn digits_49_52(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 52;
    let first_chars = &input_string[0..digits - (max_len - 4)];
    let digit_range_characters = "極";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}", handle_first_chars(first_chars), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), digits_45_48(&format!("{}{}", "0", &input_string[digits - (max_len - 8)..])));
    }
    return format!("{}{}{}", handle_first_chars(first_chars), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 52)]));
}

fn digits_53_56(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 56;
    let first_chars = &input_string[0..digits - (max_len - 4)];
    let digit_range_characters = "恒河沙";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}", handle_first_chars(first_chars), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), digits_49_52(&format!("{}{}", "0", &input_string[digits - (max_len - 8)..])));
    }
    return format!("{}{}{}", handle_first_chars(first_chars), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 56)]));
}

fn digits_57_60(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 60;
    let first_chars = &input_string[0..digits - (max_len - 4)];
    let digit_range_characters = "阿僧祇";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}", handle_first_chars(first_chars), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), digits_53_56(&format!("{}{}", "0", &input_string[digits - (max_len - 8)..])));
    }
    return format!("{}{}{}", handle_first_chars(first_chars), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 60)]));
}

fn digits_61_64(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 64;
    let first_chars = &input_string[0..digits - (max_len - 4)];
    let digit_range_characters = "那由他";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}", handle_first_chars(first_chars), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), digits_57_60(&format!("{}{}", "0", &input_string[digits - (max_len - 8)..])));
    }
    return format!("{}{}{}", handle_first_chars(first_chars), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 64)]));
}

fn digits_65_68(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 68;
    let first_chars = &input_string[0..digits - (max_len - 4)];
    let digit_range_characters = "不可思議";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}", handle_first_chars(first_chars), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), digits_61_64(&format!("{}{}", "0", &input_string[digits - (max_len - 8)..])));
    }
    return format!("{}{}{}", handle_first_chars(first_chars), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 68)]));
}

fn digits_69_72(input_string: &str) -> String {
    let digits = input_string.len();
    let max_len = 72;
    let first_chars = &input_string[0..digits - (max_len - 4)];
    let digit_range_characters = "無量大数";

    if &input_string[digits - (max_len - 4)..digits - (max_len - 8)] != "0000" {
        return format!("{}{}{}{}", handle_first_chars(first_chars), digit_range_characters, digits_4(&input_string[digits - (max_len - 4)..digits - (max_len - 8)], true), digits_65_68(&format!("{}{}", "0", &input_string[digits - (max_len - 8)..])));
    }
    return format!("{}{}{}", handle_first_chars(first_chars), digit_range_characters, trim_and_recurse(&input_string[digits - (max_len - 4)..digits - (max_len - 72)]));
}
