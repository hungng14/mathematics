const USERNAME: &'static str = "shone";
const PASSWORD: &'static str = "shone";

pub fn sign_in() -> bool {
  loop {
    let mut username = String::new();
    let mut password = String::new();
    println!("---- SIGN IN TO ADMIN ----");
    loop {
      println!("Enter Username");
      std::io::stdin()
        .read_line(&mut username)
        .expect("Expect enter username");
      username = username.replace("\n", "");
      if username.len() < 1 {
        continue;
      }
      break;
    }
    loop {
      println!("Enter Password");
      std::io::stdin()
        .read_line(&mut password)
        .expect("Expect enter password");
      password = password.replace("\n", "");
      if password.len() < 1 {
        continue;
      }
      break;
    }

    if username != USERNAME && password != PASSWORD {
      println!("Username or Password incorrect!");
      let mut option = String::new();
      loop {
        println!("Enter your option to continue!");
        println!("1. Try Sign in again      2. Quit");
        std::io::stdin()
          .read_line(&mut option)
          .expect("Expect enter option");
        option = option.replace("\n", "");
        if option.len() < 1 {
          continue;
        }
        if option == "1" {
          break;
        }
        if option == "2" {
          return false;
        }
        break;
      }
      continue;
    }
    println!("Sign in successfully!");
    return true;
  }
}
