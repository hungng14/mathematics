use sha256::digest;

fn main() {
  println!("Enter your message");
  let mut message = String::new();
  std::io::stdin().read_line(&mut message).expect("Message text is required");
  message = message.replace("\n", "");
  println!("my message {}", message);
  let hash = digest(message);
  println!("Message hashed: {}", hash);
}
