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
  fn calculate(&self, expr: &str) {
    let mut result = 0;
    let parsed_expr = convert_infix_to_postfix(expr);
    let mut stack = Vec::new();

    for splitted in parsed_expr.split_whitespace() {
      if is_numeric(splitted) {
        stack.push(splitted);
      } else {
        match splitted {
          "+" => result += sum_two_nums(&mut stack),
          _ => (),
        }
      }
    }

    self.update_output(result)
  }

  fn update_output(&self, result: i32) {
    (*self.data_view).borrow_mut().output = result.to_string();
  }
}

fn convert_infix_to_postfix(expr: &str) -> String {
  let mut parser = InfixToPostfixParser::new();
  parser.parse(expr).unwrap()
}

fn is_numeric(str: &str) -> bool {
  str.parse::<i32>().is_ok()
}

fn sum_two_nums(stack: &mut Vec<&str>) -> i32 {
  let second: i32 = stack.pop().unwrap().parse().unwrap();
  let first: i32 = stack.pop().unwrap().parse().unwrap();
  first + second
}

#[cfg(test)]
mod math_input_tests {
  use super::*;

  #[test]
  fn sum_two_numbers() {
    let data_view = Rc::new(RefCell::new(DataView { output: String::new() }));
    let input = MathInputImpl {
      data_view: Rc::clone(&data_view),
    };

    input.calculate("1 + 2");

    assert_eq!("3", (*data_view).borrow_mut().output);
  }
}
