#[path = "errors.rs"]
mod errors;

use notations::infix_to_postfix::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct DataView {
  pub output: String,
}

pub struct MathInputImpl {
  data_view: Rc<RefCell<DataView>>,
}

impl MathInputImpl {
  pub fn new(data_view: &Rc<RefCell<DataView>>) -> MathInputImpl {
    MathInputImpl {
      data_view: Rc::clone(&data_view),
    }
  }

  pub fn calculate(&self, expr: &str) {
    let parsed_expr = convert_infix_to_postfix(expr);
    let mut stack: Vec<String> = Vec::new();

    for splitted in parsed_expr.split_whitespace() {
      if is_numeric(splitted) {
        stack.push(splitted.to_string());
      } else {
        match splitted {
          "+" => sum_two_nums(&mut stack),
          "-" => minus_two_nums(&mut stack),
          "*" => multiply_two_numbers(&mut stack),
          "/" => divide_two_numbers(&mut stack),
          _ => {
            self.update_output_with_error(errors::NON_NUMERIC_ERROR);
            return;
          }
        }
      }
    }

    self.update_output(&mut stack)
  }

  fn update_output(&self, stack: &mut Vec<String>) {
    (*self.data_view).borrow_mut().output = stack.pop().unwrap();
  }

  fn update_output_with_error(&self, error_msg: &str) {
    (*self.data_view).borrow_mut().output = String::from(error_msg);
  }
}

fn convert_infix_to_postfix(expr: &str) -> String {
  let mut parser = InfixToPostfixParser::new();
  parser.parse(expr).unwrap()
}

fn is_numeric(str: &str) -> bool {
  str.parse::<i32>().is_ok()
}

fn sum_two_nums(stack: &mut Vec<String>) {
  let (first, second) = get_two_nums_from_stack(stack);
  let result = first + second;
  stack.push(result.to_string())
}

fn get_two_nums_from_stack(stack: &mut Vec<String>) -> (i32, i32) {
  let second: i32 = stack.pop().unwrap().parse().unwrap();
  let first: i32 = stack.pop().unwrap().parse().unwrap();
  return (first, second);
}

fn minus_two_nums(stack: &mut Vec<String>) {
  let (first, second) = get_two_nums_from_stack(stack);
  let result = first - second;
  stack.push(result.to_string())
}

fn multiply_two_numbers(stack: &mut Vec<String>) {
  let (first, second) = get_two_nums_from_stack(stack);
  let result = first * second;
  stack.push(result.to_string())
}

fn divide_two_numbers(stack: &mut Vec<String>) {
  let (first, second) = get_two_nums_from_stack(stack);
  let result = first / second;
  stack.push(result.to_string())
}

#[cfg(test)]
mod math_input_tests {
  use super::*;

  fn create_input_and_data_view() -> (MathInputImpl, Rc<RefCell<DataView>>) {
    let data_view = Rc::new(RefCell::new(DataView { output: String::new() }));
    let input = MathInputImpl::new(&data_view);
    (input, data_view)
  }

  #[test]
  fn output_contains_error_message_when_expression_contains_non_numeric_chars() {
    let (input, data_view) = create_input_and_data_view();

    input.calculate("a + 2");

    assert!((*data_view).borrow_mut().output.contains(errors::NON_NUMERIC_ERROR));
  }

  #[test]
  fn solve_complex_expression() {
    let (input, data_view) = create_input_and_data_view();

    input.calculate("25 + (3 * 6) / (2 + 1)");

    assert_eq!("31", (*data_view).borrow_mut().output);
  }
}
