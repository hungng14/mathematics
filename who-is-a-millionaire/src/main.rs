use postgres::{Client, Error, NoTls};
mod services;
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
        Err(_) => continue
    };
    break;
  }

  if option == 1 {

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

      std::io::stdin().read_line(&mut option).expect("Expect enter option");
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
