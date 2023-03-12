use std::io::{self, Write};

use crate::domain::template_specification::TemplateSpecificationItemType;
use crate::{application::create::interfaces::Prompt, domain::template_specification::TemplateSpecificationItem};

pub struct CliPrompt {}

impl Prompt for CliPrompt {
    fn get_answer(&self, template_specification_item: &mut TemplateSpecificationItem) {
        match &template_specification_item.item {
            TemplateSpecificationItemType::SingleChoice(choice) => {
                print!("{} ({}): ", template_specification_item.template_key, choice);
                io::stdout().flush().unwrap();

                let mut answer = String::new();
                let stdin = io::stdin();
                stdin.read_line(&mut answer).expect("issue to read line");
                let cleaned_answer = answer
                    .trim()
                    .strip_suffix("\r\n")
                    .or(answer.trim().strip_prefix('\n'))
                    .unwrap_or(answer.trim())
                    .to_string();

                if cleaned_answer.is_empty() {
                    template_specification_item.answer = choice.to_string();
                    return;
                }

                template_specification_item.answer = cleaned_answer;
            }
            TemplateSpecificationItemType::MultipleChoice(choices) => {
                println!("{} (1):", template_specification_item.template_key);
                for (index, choice) in choices.iter().enumerate() {
                    println!("  {} {}", index + 1, choice);
                }

                let mut answer = String::new();
                let stdin = io::stdin();
                stdin.read_line(&mut answer).expect("issue to read line");
                let index = answer.trim().parse::<usize>().expect("answer is not a number!");

                if index - 1 > choices.len() {
                    println!("index doesn't exist");
                    return;
                }

                template_specification_item.answer = choices[index - 1].clone();
            }
        }
    }
}
