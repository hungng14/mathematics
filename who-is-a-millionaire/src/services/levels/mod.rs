use postgres::Client;
#[path = "../../constants/mod.rs"]
mod constants;

pub fn add_level(client: &mut Client) -> bool {
  loop {
    println!("---- CREATING NEW LEVEL ----");

    let level: i32;
    let mut label = String::new();

    loop {
      let mut tmp_level = String::new();
      println!("Enter Level(must be integer number)");
      std::io::stdin()
        .read_line(&mut tmp_level)
        .expect("Expect Enter Level");
      tmp_level = tmp_level.replace("\n", "");

      level = match tmp_level.trim().parse() {
        Ok(n) => n,
        Err(_) => continue,
      };
      break;
    }

    loop {
      label = String::from("");
      println!("Enter Level Label");
      std::io::stdin()
        .read_line(&mut label)
        .expect("Expect Enter Level Label");
      label = label.replace("\n", "");
      if label.len() < 1 {
        continue;
      }
      break;
    }

    let result = client.execute(
      &format!(
        "INSERT INTO {} (level, label) VALUES($1, $2)",
        constants::schemas::LEVELS
      ),
      &[&level, &label],
    );
    match result {
      Ok(c) => {
        return c > 0;
      }
      Err(e) => {
        println!("\nAdd Failed {}\n", e);
        return false;
      }
    }
  }
}

pub fn run_level_board(client: &mut Client) {
  println!("---- Welcome to Levels admin ----\n");
  loop {
    println!("Select Option Levels Admin");
    println!("1. Create    2. Update    3. List    4. Quit");
    println!("\nEnter your option");
    let mut option = String::new();
    std::io::stdin()
      .read_line(&mut option)
      .expect("Expect choose an Option");
    option = option.replace("\n", "");
    if option != "1" && option != "2" && option != "3" && option != "4" {
      println!("Option invalid. Please try again!\n");
      continue;
    }
    if option == "4" {
      println!("Thank you. See you soon!");
      break;
    }
    if option == "1" {
      let ok = add_level(client);
      if ok {
        println!("\nInsert successfully!");
        continue;
      }
    }
    if option == "2" {
      let ok = add_level(client);
      if ok {
        println!("\nInsert successfully!");
        continue;
      }
    }

    if option == "3" {
      let ok = add_level(client);
      if ok {
        println!("\nInsert successfully!");
        continue;
      }
    }
  }
}
