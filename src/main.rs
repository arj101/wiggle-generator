use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let text = if let Some(text) = args.get(1) {
        text
    } else {
        eprintln!("Oi, gimme some text first... (`wiggle-generator [some text]`)");
        std::process::exit(65);
    };

    
    let line_len = if let Ok(val) = env::var("WIGGLE_LINE_LEN") {
        if let Ok(number) = val.parse::<usize>() {
            number
        } else { 30 }
    } else { 30 };

    let wiggle_rate = if let Ok(val) = env::var("WIGGLE_RATE") {
        if let Ok(number) = val.parse::<f64>() {
            number
        } else { 0.1 }
    } else { 0.1 };

    let line_count_parsed = if let Ok(val) = env::var("WIGGLE_LINE_COUNT") {
        if let Ok(number) = val.parse::<usize>() {
            Some(number)
        } else { None }
    } else { None };



    let output_len = if let None = line_count_parsed {
        if let Ok(val) = env::var("WIGGLE_OUTPUT_LEN") {
            if let Ok(number) = val.parse::<usize>() {
                number
            } else { 2000 }
        } else { 2000 }
    } else {
        2000
    };

    let line_count = if let Some(count) = line_count_parsed {
        count
    } else {
        output_len / line_len
    };

    let mut output_text = String::new();

    let mut x: f64 = -std::f64::consts::FRAC_PI_2;

    for _ in 0..line_count {

        let wiggle_multiplier = ( line_len / 2 ) - (text.len() / 2);
        let wiggle_offset = (line_len / 2) as isize;
        let wiggle = x.sin() * wiggle_multiplier as f64;
        let word_pos = (wiggle.floor() as isize - (text.len() / 2) as isize) + wiggle_offset;

        let mut line = String::new();

        line.push_str(&" ".repeat(word_pos as usize));
        line.push_str(text);
        line.push('\n');

        output_text.push_str(&line);

        x += wiggle_rate;
    }

    println!("{}", output_text);
}
