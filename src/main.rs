use rand::Rng; // 0.8.5

fn get_input() -> String {
    let mut word = String::new();
    std::io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");
    return word;
}

fn user_hand_manager() -> String {
    // ユーザの操作制御
    let input = get_input();
    let input_str = input.trim(); //改行キーの削除
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
    //CPUの制御
    let hand_num = rand::thread_rng().gen_range(0..3); //操作判断のため、乱数生成
    match hand_num { //各値によって操作を決定
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
    //ユーザとCPUの操作を比較し、すべてのパターンを網羅
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
    //各操作に対して適当な絵文字を返す
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
        // 入力の指示
        println!("please your hand.");
        println!("-Rock: r \n-Paper: p \n-Scissors: s");
        //ユーザとCPUの操作の決定
        let user_hand = user_hand_manager();
        let cpu_hand = cpu_hand_manager();

        let winner = judge(&user_hand, &cpu_hand); //勝者を判断
        //それぞれの操作を事前に表示
        println!("\nYou take {}.", to_emoji(user_hand));
        println!("Cpu take {}.", to_emoji(cpu_hand));
        
        //結果表示
        if winner == "user" {
            println!("You are win!");
        }else if winner == "cpu" {
            println!("You are lose...");
        }else {
            println!("We are draw.\nagain.");
            continue; //drawの場合はもう一度やるのでループをスキップ
        }
        println!("-----------------\n");
        //繰り返し制御
        println!("again? y/n");
        let again = get_input();
        let again_str = again.trim(); //改行キーの削除
        if again_str == "n" {
            break; //nが入力された場合のみプログラム終了
        }
    }
}