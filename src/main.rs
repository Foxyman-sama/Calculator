mod math_input;
mod ui;

struct Mock;

impl ui::MathInput for Mock {
  fn calculate(&self, expr: &str) -> i128 {
    println!("{}", expr);
    2
  }
}

fn main() {
  let interface = ui::UI::new("hello, world!", 1280, 720, Box::new(Mock));

  interface.run();
}
