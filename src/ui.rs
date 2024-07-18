use std::borrow::{Borrow, BorrowMut};

use glow::HasContext;
use imgui::Context;
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use sdl2::{
  event::Event,
  video::{GLProfile, Window},
};

pub trait MathInput {
  fn calculate(&self, expr: &str) -> i32;
}

struct SDLWindowBackend<'a> {
  context: &'a mut Context,
  platform: &'a mut SdlPlatform,
  renderer: &'a mut AutoRenderer,
}

pub struct CalculatorWindow {
  title: &'static str,
  width: u32,
  height: u32,
  input: Box<dyn MathInput>,
}

impl CalculatorWindow {
  pub fn new(title: &'static str, width: u32, height: u32, input: Box<dyn MathInput>) -> CalculatorWindow {
    CalculatorWindow {
      title,
      width,
      height,
      input,
    }
  }

  pub fn show(&self) {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);

    let window = video_subsystem
      .window(&self.title, self.width, self.height)
      .allow_highdpi()
      .opengl()
      .position_centered()
      .resizable()
      .build()
      .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();
    window.subsystem().gl_set_swap_interval(1).unwrap();

    let gl = glow_context(&window);

    let mut context = Context::create();
    context.set_ini_filename(None);
    context.set_log_filename(None);

    let mut platform = SdlPlatform::init(&mut context);
    let mut renderer = AutoRenderer::initialize(gl, &mut context).unwrap();

    let mut window_backend = SDLWindowBackend {
      context: &mut context,
      platform: &mut platform,
      renderer: &mut renderer,
    };

    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
      for event in event_pump.poll_iter() {
        /* pass all events to imgui platfrom */
        window_backend
          .platform
          .handle_event(&mut window_backend.context, &event);

        if let Event::Quit { .. } = event {
          break 'main;
        }
      }

      /* call prepare_frame before calling imgui.new_frame() */
      window_backend
        .platform
        .prepare_frame(&mut window_backend.context, &window, &event_pump);

      show_main_window(&mut window_backend, &self.input);
      render(&mut window_backend, &window);
    }
  }
}

fn show_main_window(window_backend: &mut SDLWindowBackend, input: &Box<dyn MathInput>) {
  let ui = window_backend.context.new_frame();
  const WINDOW_TITLE: &'static str = "Calculator";
  const WINDOW_WIDTH: f32 = 310.;
  const WINDOW_HEIGHT: f32 = 260.;

  let mut result = String::new();

  ui.window(WINDOW_TITLE)
    .size([WINDOW_WIDTH, WINDOW_HEIGHT], imgui::Condition::FirstUseEver)
    .build(|| {
      const INPUT_FIELD_WIDTH: f32 = WINDOW_WIDTH;
      const INPUT_FIELD_HEIGHT: f32 = 60.;
      let win_size = ui.window_size();
      let cursor_pos = ui.cursor_pos();
      ui.set_cursor_pos([(win_size[0] - INPUT_FIELD_WIDTH) * 0.5, cursor_pos[1]]);
      ui.input_text_multiline("###input", &mut result, [INPUT_FIELD_WIDTH, INPUT_FIELD_HEIGHT])
        .build();

      let cursor_pos = ui.cursor_pos();
      ui.set_cursor_pos([(win_size[0] - INPUT_FIELD_WIDTH) * 0.5, cursor_pos[1]]);
      ui.columns(3, "hello", false);
      for i in 1..=9 {
        ui.button_with_size(i.to_string(), [INPUT_FIELD_WIDTH / 3., 50.]);
        ui.next_column();
      }
    });
}

fn render(window_backend: &mut SDLWindowBackend, window: &Window) {
  let draw_data = window_backend.context.render();

  unsafe { window_backend.renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
  window_backend.renderer.render(draw_data).unwrap();

  window.gl_swap_window();
}

fn glow_context(window: &Window) -> glow::Context {
  unsafe { glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _) }
}
