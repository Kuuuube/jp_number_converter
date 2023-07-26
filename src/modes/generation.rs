use std::time::Instant;

use crate::{Settings, StepType};
use crate::{banknote_daiji, daiji, hiragana, kanji};

pub fn generation_mode(settings: Settings) {
    let hiragana_convert = settings.format_string.contains("{hiragana}");
    let kanji_convert = settings.format_string.contains("{kanji}");
    let banknote_daiji_convert = settings.format_string.contains("{banknote_daiji}");
    let daiji_convert = settings.format_string.contains("{daiji}");

    println!("Generation Mode");

    //benchmarking code
    let start_time = Instant::now();

    let mut output_file = match std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(&settings.output) {
        Ok(ok) => ok,
        Err(_) => panic!("Couldn't open file.")
    };

    let mut generated_string = String::default();
    let start_number = settings.range.0.parse::<f64>().unwrap_or_default();
    let end_number = settings.range.1.parse::<f64>().unwrap_or_default();
    let mut i = start_number;

    while i < end_number + settings.step {
        let input_string = clean_f64_to_string(i, settings.step_decimal_len);
        let hiragana_output = if hiragana_convert {
            hiragana::convert_number(&input_string)
        } else {
            "".to_string()
        };
        let kanji_output = if kanji_convert {
            kanji::convert_number(&input_string)
        } else {
            "".to_string()
        };
        let banknote_daiji_output = if banknote_daiji_convert {
            banknote_daiji::convert_number(&input_string)
        } else {
            "".to_string()
        };
        let daiji_output = if daiji_convert {
            daiji::convert_number(&input_string)
        } else {
            "".to_string()
        };
        generated_string += &format!("{}", settings.format_string.replace("{arabic}", &input_string).replace("{hiragana}", &hiragana_output).replace("{kanji}", &kanji_output).replace("{banknote_daiji}", &banknote_daiji_output).replace("{daiji}", &daiji_output).replace("\\n", "\n"));

        match settings.step_type {
            StepType::Add => i += settings.step,
            StepType::Multiply => i *= settings.step,
            StepType::Exponent => i = f64::powf(i, settings.step)
        }
    }

    match std::io::Write::write(&mut output_file, generated_string.as_bytes()) {
        Ok(_) => (),
        Err(_) => println!("Failed to write file.")
    };

    //benchmarking code
    let time_elapsed = start_time.elapsed();
    println!("Generated in: {time_elapsed:.6?}");
}

fn clean_f64_to_string(float: f64, rounding: usize) -> String {
    let rounded = format!("{:.rounding$}", float);
    if rounded.contains(".") {
        return rounded.trim_end_matches("0").trim_end_matches(".").to_string();
    }
    return rounded;
}