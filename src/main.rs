use rand::Rng; // 0.8.5

fn get_input() -> String {
    let mut word = String::new();
    std::io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");
    return word;
}

fn user_hand_manager() -> String {
    let input = get_input();
    let input_str = input.trim();
    match input_str {
        "r" => {
            return "Rock".to_string();
        }
        "p" => {
            return "Paper".to_string();
        }
        _ => {
            return "Scissors".to_string();
        }
    }
}

fn cpu_hand_manager() -> String {
    let hand_num = rand::thread_rng().gen_range(0..3);
    match hand_num {
        0 => {
            return "Rock".to_string();
        }
        1 => {
            return "Paper".to_string();
        }
        _ => {
            return "Scissors".to_string();
        }
    }
}

fn judge(user_hand: &str, cpu_hand: &str) -> String {
    if user_hand == "Rock" {
        if cpu_hand == "Rock" {
            return "Draw".to_string();
        }else if cpu_hand == "Paper" {
            return "cpu".to_string();
        }else {
            return  "user".to_string();
        }
    }else if user_hand == "Paper" {
        if cpu_hand == "Rock" {
            return "user".to_string();
        }else if cpu_hand == "Paper" {
            return "Draw".to_string();
        }else {
            return  "cpu".to_string();
        }
    }else if user_hand == "Scissors" {
        if cpu_hand == "Rock" {
            return "cpu".to_string();
        }else if cpu_hand == "Paper" {
            return  "user".to_string();
        }else {
            return "Draw".to_string();
        }
    }else {
        return "".to_string();
    }
}

fn to_emoji(hand_type: String) -> char{
    let hand_type_str = hand_type.as_str();
    match hand_type_str {
        "Rock" => {
            return '\u{270A}';
        }
        "Paper" => {
            return '\u{270B}';
        }
        _ => {
            return  '\u{270C}';
        }
    }
}

fn main() {
    loop {
        println!("please your hand.");
        println!("-Rock: r \n-Paper: p \n-Scissors: s");
        let user_hand = user_hand_manager();
        let cpu_hand = cpu_hand_manager();

        let winner = judge(&user_hand, &cpu_hand);
        println!("\nYou take {}.", to_emoji(user_hand));
        println!("Cpu take {}.", to_emoji(cpu_hand));
        
        if winner == "user" {
            println!("You are win!");
        }else if winner == "cpu" {
            println!("You are lose...");
        }else {
            println!("We are draw.\nagain.");
            continue;
        }
        println!("-----------------\n");
        println!("again? y/n");
        let again = get_input();
        let again_str = again.trim();
        if again_str == "n" {
            break;
        }
    }
}