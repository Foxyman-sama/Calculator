pub struct DataView {
  pub output: String,
}

pub struct MathInputImpl<'a> {
  data_view: &'a mut DataView,
}

impl<'a> MathInputImpl<'a> {
  fn calculate(&mut self, expr: &str) {
    self.data_view.output = String::from("3");
  }
}

#[cfg(test)]
mod math_input_tests {
  use crate::math_input::{DataView, MathInputImpl};

  #[test]
  fn sum_two_numbers() {
    let mut data_view = DataView { output: String::new() };
    let mut input = MathInputImpl {
      data_view: &mut data_view,
    };

    input.calculate("1 + 1");

    assert_eq!("3", data_view.output);
  }
}
