fn main() {
    let string = String::from("Hello");
    for (i, word) in string.as_bytes().iter().enumerate() {
      println!("{}: {}",i, word);
    }
}



