use std::{vec};

use postgres::Client;
// use std::collections::HashMap;
use uuid::Uuid;
#[path = "../../constants/mod.rs"]
mod constants;

#[derive(Debug)]
struct Level {
    index: usize,
    label: String,
}

#[derive(Debug)]
pub struct QuestionDetail {
    #[allow(dead_code)]
    pub option: String,
    #[allow(dead_code)]
    pub is_correct: bool,
}

#[derive(Debug)]
pub struct Question {
    pub id: Uuid,
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub level: i32,
    pub options: Vec<QuestionDetail>,
}

fn get_answer(label: &'static str) -> String {
    let mut output = String::new();
    loop {
        println!("Enter Answer {}", label);
        std::io::stdin()
            .read_line(&mut output)
            .expect("Expect Enter Answer");
        output = output.replace("\n", "");

        if output.len() < 1 {
            continue;
        }
        break;
    }
    output
}

fn insert_question_detail(
    client: &mut Client,
    question_id: Uuid,
    option: String,
    is_correct: bool,
) {
    client
        .execute(
            &format!(
                "INSERT INTO {} (question_id, option, is_correct) VALUES($1, $2, $3)",
                constants::schemas::QUESTION_DETAIL
            ),
            &[&question_id, &option, &is_correct],
        )
        .unwrap();
}

fn add_question(client: &mut Client) -> bool {
    println!("---- CREATING NEW QUESTION ----\n");
    // let mut Levels: HashMap<usize, Row> = HashMap::new();
    let mut idx = 0;
    let mut label_levels: Vec<Level> = vec![];
    let query_levels = client
        .query("SELECT id, label, level FROM levels", &[])
        .unwrap();
    for row in query_levels {
        label_levels.push(Level {
            index: idx,
            label: row.get(1),
        });
        // Levels.insert(idx, row);
        idx += 1;
    }

    let question_name: String;
    let level_index: i32;
    loop {
        let mut tmp_question_name = String::new();
        println!("Enter Question Name");
        std::io::stdin()
            .read_line(&mut tmp_question_name)
            .expect("Expect Enter Question Name");
        tmp_question_name = tmp_question_name.replace("\n", "");

        question_name = match tmp_question_name.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        break;
    }

    loop {
        let mut tmp = String::new();
        println!("\nSelect level for this question");
        for row in &label_levels {
            println!("{}: {}", row.index, row.label);
        }
        std::io::stdin()
            .read_line(&mut tmp)
            .expect("Expect Enter Level Index");
        level_index = match tmp.trim().parse() {
            Ok(idx) => {
                let is_match = match label_levels.get(idx as usize) {
                    Some(_d) => true,
                    None => continue,
                };
                if !is_match {
                    continue;
                }
                idx
            }
            Err(_) => continue,
        };
        break;
    }

    let option_a = get_answer("A");
    let option_b = get_answer("B");
    let option_c = get_answer("C");
    let option_d = get_answer("D");

    let mut answer: String;
    loop {
        println!("Enter answer for this question!");
        answer = get_answer("Correct");
        if answer != option_a && answer != option_b && answer != option_c && answer != option_d {
            continue;
        }
        break;
    }

    let result = client.query(
        &format!(
            "INSERT INTO {} (level, name) VALUES($1, $2) returning id, name",
            constants::schemas::QUESTION
        ),
        &[&level_index, &question_name],
    );
    match result {
        Ok(inserted) => {
            for row in inserted {
                let question_id: Uuid = row.get("id");
                insert_question_detail(client, question_id, option_a.clone(), answer == option_a);
                insert_question_detail(client, question_id, option_b.clone(), answer == option_b);
                insert_question_detail(client, question_id, option_c.clone(), answer == option_c);
                insert_question_detail(client, question_id, option_d.clone(), answer == option_d);
            }
            return true;
        }
        Err(e) => {
            println!("\nAdd Failed {}\n", e);
            return false;
        }
    }
}

pub struct QueryListQuestion {
  pub level: i32,
  pub random: bool
}

pub fn list_question(client: &mut Client, query: Option<QueryListQuestion>) -> Vec<Question> {
    let condition = match &query {
      Some(q) => {
        format!("WHERE level={}", q.level)
      },
      None => "".to_string()
    };

    let query_str = format!("SELECT * FROM question RIGHT JOIN question_detail ON question.id=question_detail.question_id {}", condition);
    let query = client.query(&query_str, &[]).unwrap();
    let mut questions: Vec<Question> = vec![];

    for row in query {
        let question_id: Uuid = row.get("id");
        let mut idx_found: i32 = -1;
        for (idx, q) in questions.iter().enumerate() {
            if q.id.eq(&question_id) {
                idx_found = idx as i32;
                break;
            }
        }

        let question_detail = QuestionDetail {
            option: row.get("option"),
            is_correct: match row.get("is_correct") {
                Some(v) => v,
                None => false,
            },
        };

        if idx_found == -1 {
            let question = Question {
                id: question_id,
                name: row.get("name"),
                level: row.get("level"),
                options: vec![question_detail],
            };
            questions.push(question);
            continue;
        }

        match questions.get_mut(idx_found as usize) {
            Some(q) => q.options.push(question_detail),
            None => {
                let question = Question {
                    id: question_id,
                    name: row.get("name"),
                    level: row.get("level"),
                    options: vec![question_detail],
                };
                questions.push(question);
            }
        }
    }
    questions
}

pub fn run_question_board(client: &mut Client) {
    println!("---- Welcome to Question Board ----\n");
    loop {
        println!("Select Option Question Admin");
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
            let ok = add_question(client);
            if ok {
                println!("\nInsert successfully!\n");
                continue;
            }
        }

        if option == "3" {
            let questions = list_question(client, None);
            println!("List Questions:");
            println!("{:#?}", questions);
        }
    }
}
