use std::collections::HashMap;

fn main() {
  println!("Enter usize number");
  let mut number = String::new();
  std::io::stdin().read_line(&mut number).expect("Please enter usize number");
  let number: u128 = match number.trim().parse() {
    Ok(n) => n,
    Err(_) => 0
  };
  let output = convert_decimal_to_hexa(number);
  println!("Result = {}", output);
}

fn get_result_of_div(number: u128, remaining_numbers: &mut Vec<u8>) {
  let remaining = number % 16;
  let res_int = number / 16;
  remaining_numbers.insert(0, remaining as u8);
  if res_int > 0 {
    get_result_of_div(res_int, remaining_numbers);
  }
}

fn convert_decimal_to_hexa(number: u128) -> String {
  if number == 0 {
    return "0x00".to_string();
  }
  let mut hexa_numbers: HashMap<u8, String> = HashMap::new();
  hexa_numbers.insert(0, "0".to_string());
  hexa_numbers.insert(1, "1".to_string());
  hexa_numbers.insert(2, "2".to_string());
  hexa_numbers.insert(3, "3".to_string());
  hexa_numbers.insert(4, "4".to_string());
  hexa_numbers.insert(5, "5".to_string());
  hexa_numbers.insert(6, "6".to_string());
  hexa_numbers.insert(7, "7".to_string());
  hexa_numbers.insert(8, "8".to_string());
  hexa_numbers.insert(9, "9".to_string());
  hexa_numbers.insert(10, "A".to_string());
  hexa_numbers.insert(11, "B".to_string());
  hexa_numbers.insert(12, "C".to_string());
  hexa_numbers.insert(13, "D".to_string());
  hexa_numbers.insert(14, "E".to_string());
  hexa_numbers.insert(15, "F".to_string());

  let mut array_number: Vec<u8> = vec![];
  get_result_of_div(number, &mut array_number);
  println!("{:?}", array_number);
  let mut output: String = String::from("Ox");
  array_number.iter().for_each(|n| {
    let k = hexa_numbers.get(&n).unwrap().clone();
    output.push_str(&k)
  });
  return output;
}
