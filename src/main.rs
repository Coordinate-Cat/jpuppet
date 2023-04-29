use std::io::{self, Write};

mod data;

fn main() {
    // 変数の定義
    let last_names = &data::lastnames::LAST_NAMES;
    let first_names_female = ["ナオコ", "ヨーコ", "サクラ"];
    let first_names_male: [&str; 3] = ["アキラ", "テツオ", "コナン"];
    let mut ages: Vec<Option<String>> = vec![None; last_names.len()];
    let genders = ["Female", "Male", "Other"];

    // ターミナルの画面幅を取得して、それに合わせて線を引く
    let width = term_size::dimensions().unwrap().0;

    // 名前の選択
    println!();
    println!("Please select your last name from the following list.");

    // 配列の要素を横5列縦20行で表示する
    // 例: 1: 佐藤  2: 鈴木  3: 高橋  4: 田中  5: 渡辺
    for (i, last_name) in last_names.iter().enumerate() {
        // nameの文字数によって、スペースの数を変える
        if last_name.chars().count() == 3 && last_names[i + 1].chars().count() == 2 {
            // 3文字の場合、2文字分のスペースを左に追加する
            print!("{:>3}: {:<3} ", i + 1, last_name);
        } else if last_name.chars().count() == 1 && last_names[i + 1].chars().count() == 1 {
            // 1文字の場合、2文字分のスペースを左に追加する
            print!("{:>3}: {:<6}", i + 1, last_name);
        } else {
            print!("{:>3}: {:<4} ", i + 1, last_name);
        }

        // 5列ごとに改行する
        if (i + 1) % 5 == 0 {
            println!();
        }
    }

    // 入力を促し、名前を取得する
    let last_name = loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed = input.trim();
        if let Ok(index) = trimmed.parse::<usize>() {
            if index > 0 && index <= last_names.len() {
                break last_names[index - 1].to_owned();
            }
            // もしindexが0以下もしくは配列の長さを超えていたら、エラーを表示する
            if index <= 0 {
                println!("Index must be greater than 0.");
            } else {
                println!("Index must be less than or equal to {}.", last_names.len());
            }
        }
        // もしindexが空白だったら、数字でなかったらエラーを表示する
        if trimmed.is_empty() {
            println!("Index cannot be empty.");
        } else {
            println!("Index must be a number.");
        }
    };

    // 年齢の入力
    println!();
    println!("Please enter your age:");

    let age = loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed = input.trim().to_owned();
        if !trimmed.is_empty() {
            break Some(trimmed);
        }
        println!("Age cannot be empty. Please try again.");
    };

    // 年齢を配列に追加する
    ages[last_names.iter().position(|&n| n == last_name).unwrap()] = age;

    // 性別の選択
    println!();
    println!("Please select your gender from the following list:");

    for (i, gender) in genders.iter().enumerate() {
        println!("{}: {}", i + 1, gender);
    }

    // 入力を促し、性別を取得する
    let gender = loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed = input.trim();
        if let Ok(index) = trimmed.parse::<usize>() {
            if index <= genders.len() {
                break genders[index - 1].to_owned();
            }
        }
        println!("Invalid input. Please try again.");
    };

    // 性別によって、first_namesを選択する
    let first_names = match gender.as_str() {
        "Female" => &first_names_female,
        "Male" => &first_names_male,
        _ => {
            println!("First name selection is only available for Female and Male genders.");
            return;
        }
    };

    println!();
    println!("Please select your first name from the following list:");

    for (i, first_name) in first_names.iter().enumerate() {
        println!("{}: {}", i + 1, first_name);
    }

    let first_name = loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed = input.trim();
        if let Ok(index) = trimmed.parse::<usize>() {
            if index > 0 && index <= first_names.len() {
                break first_names[index - 1].to_owned();
            }
            println!("Invalid input. Please try again.");
        }
    };

    // ################ 入力された情報を表示する ################
    println!("{}", "#".repeat(width));
    println!("Thank you for your input!");
    println!("Persona has been generated.");
    println!();

    println!("Last Name: {}", last_name);
    println!("First Name: {}", first_name);
    // 年齢が入力されていたら、年齢を表示する
    if let Some(age) = ages
        .iter()
        .find(|&a| a.is_some())
        .map(|a| a.as_ref().unwrap())
    {
        println!("Age: {} 歳", age);
    } else {
        println!("Age: (not provided)");
    }
    println!("Gender: {}", gender);

    println!("{}", "#".repeat(width));
    // ####################################################
}
