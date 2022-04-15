use postgres::{Client, Error, NoTls};
mod services;
use services::question::{list_question, QueryListQuestion};

fn challenge_of_round(level: i32, client: &mut Client) -> bool {
    let easy_questions = list_question(client, Some(QueryListQuestion { level, random: true }));
    let total_questions = easy_questions.len();
    let mut total_question_passed: usize = 0;
    let mut passed_easy_level = true;
    let mut current_question: usize = 0;

    while total_question_passed < total_questions {
        let question = easy_questions.get(current_question).unwrap();
        let mut your_answer = String::new();
        println!("Question {}: {}", current_question + 1, question.name);
        println!("A. {}", question.options.get(0).unwrap().option);
        println!("B. {}", question.options.get(1).unwrap().option);
        println!("C. {}", question.options.get(2).unwrap().option);
        println!("D. {}", question.options.get(3).unwrap().option);
        println!("");
        print!("Your Answer: ");
        println!("");
        std::io::stdin()
            .read_line(&mut your_answer)
            .expect("Expect enter your answer");
        your_answer = your_answer.replace("\n", "");
        match your_answer.to_lowercase().as_str() {
            "a" => {
                if !question.options.get(0).unwrap().is_correct {
                    passed_easy_level = false;
                }
            }
            "b" => {
                if !question.options.get(1).unwrap().is_correct {
                    passed_easy_level = false;
                }
            }
            "c" => {
                if !question.options.get(2).unwrap().is_correct {
                    passed_easy_level = false;
                }
            }
            "d" => {
                if !question.options.get(3).unwrap().is_correct {
                    passed_easy_level = false;
                }
            }
            _ => {
                continue;
            }
        }

        if !passed_easy_level {
            break;
        }
        current_question += 1;
        total_question_passed += 1;
        println!("Congratulations");
    }
    if !passed_easy_level {
        println!("Oops!. Unlucky, Hope to see you next time!");
        return false;
    }
    println!("Wow!. Congratulations, You passed this round");
    return true;
}

fn run_countdown_to_start_game() {
  let mut countdown: u8 = 5;
  while countdown > 0 {
      println!("Game will be started in {}s", countdown);
      let t = std::time::Duration::from_millis(1000);
      std::thread::sleep(t);
      countdown -= 1;
  }
}
fn main() -> Result<(), Error> {
    // let mut connection = types::SQLConnection {
    //   client: Client::connect(
    //     "postgresql://root:123456@172.22.0.3:5432/who_is_a_millionaire",
    //     NoTls,
    //   )?
    // };
    let mut client = Client::connect(
        "postgresql://root:123456@172.22.0.2:5432/who_is_a_millionaire",
        NoTls,
    )?;

    println!("Welcome to Who Is A Millionaire");

    let option: u8;

    loop {
        let mut tmp_option = String::new();
        println!("Select Option");
        println!("1. Play Game");
        println!("2. Register to Play");
        println!("3. Sign In Admin");
        println!("Enter your option");
        std::io::stdin()
            .read_line(&mut tmp_option)
            .expect("Failed to input username");
        tmp_option = tmp_option.replace("\n", "");
        if tmp_option != "1" && tmp_option != "2" && tmp_option != "3" {
            continue;
        }
        option = match tmp_option.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        break;
    }

    if option == 1 {
        println!("Enter Your Name: ");
        let mut player_name = String::new();
        loop {
            std::io::stdin()
                .read_line(&mut player_name)
                .expect("Expect enter player name");
            player_name = player_name.replace("\n", "");
            if player_name.len() < 5 {
                println!("Your name must be from 5 characters, Please try again.");
                continue;
            }
            break;
        }
        println!("Hello {}, welcome to Who Is A Million", player_name);
        println!("");

        let mut agree_option = String::new();
        loop {
            println!("Are you ready?");
            println!("Yes   or    No");
            std::io::stdin()
                .read_line(&mut agree_option)
                .expect("Expect Enter Option");
            agree_option = agree_option.replace("\n", "").to_lowercase();
            if agree_option == String::from("y") || agree_option == String::from("yes") {
                println!("Lets go");
                break;
            }
            if agree_option == String::from("n") || agree_option == String::from("no") {
                println!("Oh, Thank you, Hope to see you soon. Bye Bye");
                break;
            }
            continue;
        }

        loop {
          run_countdown_to_start_game();
          println!("--- EASY LEVEL ----");
          println!("Reward for this round are: 200 USDT");
          let passed = challenge_of_round(2, &mut client);
          if !passed {
            println!("---Unlucky. Thank you ----");
            break;
          }
  
          run_countdown_to_start_game();
          println!("--- MEDIUM LEVEL ----");
          println!("Reward for this round are: 1.000.000 USDT");
          let passed = challenge_of_round(1, &mut client);
          if !passed {
            println!("---Unlucky. Thank you ----");
            break;
          }
  
          run_countdown_to_start_game();
          println!("--- HARD LEVEL ----");
          println!("Reward for this round are: 10.000.000 USDT");
          let passed = challenge_of_round(0, &mut client);
          if !passed {
            println!("---Unlucky. Thank you ----");
            break;
          }
          println!("WOW. Congratulations, You Are Victory!");
          break;
        }
    }

    if option == 2 {
        let ok = services::register::run(&mut client);
        if ok {
            println!("Register successfully!");
        } else {
            println!("Register Failed!");
        }
    }

    if option == 3 {
        let ok = services::sign_in_admin::sign_in();
        if ok {
            println!("Select Option Admin");
            println!("1. Levels");
            println!("2. Questions");
            println!("\nEnter your option");
            let mut option = String::new();

            std::io::stdin()
                .read_line(&mut option)
                .expect("Expect enter option");
            option = option.replace("\n", "");
            if option == "1" {
                services::levels::run_level_board(&mut client);
            }

            if option == "2" {
                services::question::run_question_board(&mut client);
            }
        }
    }

    Ok(())
}
