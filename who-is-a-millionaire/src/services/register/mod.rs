use postgres::Client;

#[path="../../constants/mod.rs"]
mod constants;

pub fn run(client: &mut Client) -> bool {
  let mut username = String::new();
  let age: i32;
  loop {
    println!("---- REGISTER USER ----");
    println!("Please enter your name");
    std::io::stdin()
      .read_line(&mut username)
      .expect("Failed to input username");
    username = username.replace("\n", "");
    if username.len() < 1 {
      continue;
    }
    break;
  }
  loop {
    let mut temp_age = String::new();
    println!("Please enter your age");
    std::io::stdin()
      .read_line(&mut temp_age)
      .expect("Failed to input age");
    temp_age = temp_age.replace("\n", "");
    age = match temp_age.trim().parse() {
      Ok(n) => n,
      Err(_) => continue,
    };
    break;
  }

  println!(
    "Welcome {}, {} years old to Who Is A Millionaire",
    username, age
  );

  let result = client.execute(
    &format!("INSERT INTO {:?} (fullname, age) VALUES ($1, $2)", constants::schemas::USERS), 
    &[&username, &age]
  );

  match result {
    Ok(c) => {
      println!("added {}", c);
      return c > 0
    },
    Err(e) => {
      println!("Add failed {}", e);
      return false
    }
  }

}