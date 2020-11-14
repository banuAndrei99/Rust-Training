// use rand::Rng;
use std::io::stdin;


fn if_statement(){
    let temp = 35;

    if temp > 30 {
        println!("Really hot outside!")
    } else if temp < 10 {
        println!("Really cold outside!")
    }
    else{
        println!("Temperature is ok!")
    }

    // inline if
    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("Today is {}", day);
}

fn while_and_loop(){
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 {
            continue;
        }
        println!("x = {}", x)
    }

    let mut y = 1;
    loop {  // while true
        y *= 2;
        println!("y = {}", y);
        if y == 1 << 10 {
            break;
        }
    }
}

fn for_loop() {
    for x in 1..11 {
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("on pos {} we have y = {}", pos, y);
    }
}

fn match_statement() {
    let country_code = 44;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",  // 1..=1000 -> inclusive range
        _ => "invalid"  // _ matches everything that was not matched yet
    };
    println!("the country with code {} is {}", country_code, country);

    let x = false;
    let s = match x {
        true => "yes",
        false => "no"
    };

    println!("{}", s)

    // match is an exhaustive switch case
}

enum State {
    Locked,
    Failed,
    Unlocked
}


fn combination_lock() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end())
                    }
                    Err(_) => continue
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if ! code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("Failed");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("Unlocked");
                return;
            }
        }
    }
}

fn main() {
    if_statement();
    while_and_loop();
    for_loop();
    match_statement();
    combination_lock()
}
