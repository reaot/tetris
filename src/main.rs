use pixels::{Pixels, SurfaceTexture};
use std::time::{Duration, Instant};
use winit::{
  dpi::{PhysicalSize, Size},
  event::{
    ElementState, Event, KeyboardInput, StartCause, VirtualKeyCode,
    WindowEvent,
  },
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

mod glyphs;
use glyphs::glyph_x2;
mod tetris;
use tetris::{Field, Glyph, InputField, FIELD_HEIGHT, FIELD_WIDTH};

const PIECE_DRAW_SIZE: usize = 16;
const WIDTH: usize = (FIELD_WIDTH + 4 * 2) * PIECE_DRAW_SIZE;
const HEIGHT: usize = FIELD_HEIGHT * PIECE_DRAW_SIZE;

const PERIOD_MS: u32 = 200;

impl Field {
  pub fn draw(&self, frame: &mut [u8]) {
    let array = self.draw_array();
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
      let x = i % WIDTH;
      let y = i / WIDTH;

      let id_x = x / (WIDTH / (FIELD_WIDTH + 4 * 2));
      let id_y = y / (HEIGHT / FIELD_HEIGHT);

      if id_y * (FIELD_WIDTH + 4 * 2) + id_x
        < (FIELD_WIDTH + 4 * 2) * FIELD_HEIGHT
      {
        let glyph = &array[id_y * (FIELD_WIDTH + 4 * 2) + id_x];
        match glyph {
          Glyph::Color(c) => {
            let c: [u8; 4] = (*c).into();
            pixel.copy_from_slice(&c);
          }
          Glyph::Number(n) => {
            let loc_x = x - id_x * PIECE_DRAW_SIZE;
            let loc_y = y - id_y * PIECE_DRAW_SIZE;
            if glyph_x2(*n as u32)[loc_y][loc_x] {
              pixel.copy_from_slice(&[!0, !0, !0, !0]);
            } else {
              pixel.copy_from_slice(&[0, 0, 0, !0]);
            }
          }
        }
      }
      if x == 4 * PIECE_DRAW_SIZE
        || x == (4 + FIELD_WIDTH) * PIECE_DRAW_SIZE - 1
      {
        pixel.copy_from_slice(&[!0, !0, !0, !0]);
      }
    }
  }
}

fn main() {
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_resizable(false)
    .with_title("tetris")
    .with_inner_size(Size::Physical(PhysicalSize {
      width: WIDTH as u32,
      height: HEIGHT as u32,
    }))
    .build(&event_loop)
    .unwrap();

  let window_size = window.inner_size();
  let surface_texture =
    SurfaceTexture::new(window_size.width, window_size.height, &window);
  let mut pixels =
    Pixels::new(window_size.width, window_size.height, surface_texture)
      .unwrap();

  let period = Duration::new(1, 0) / 1000 * PERIOD_MS;

  let mut field = Field::new();

  event_loop.run(move |event, _, control_flow| match event {
    Event::WindowEvent { event, window_id }
      if window_id == window.id() =>
    {
      match event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::KeyboardInput {
          input:
            KeyboardInput {
              state: ElementState::Pressed,
              virtual_keycode: Some(keycode),
              ..
            },
          ..
        } => match keycode {
          VirtualKeyCode::Left => {
            field.process_input(InputField::Left);
            window.request_redraw();
          }
          VirtualKeyCode::Right => {
            field.process_input(InputField::Right);
            window.request_redraw();
          }
          VirtualKeyCode::Up => {
            field.process_input(InputField::Rotate);
            window.request_redraw();
          }
          VirtualKeyCode::Down => {
            field.drop_figure();
            window.request_redraw();
          }
          VirtualKeyCode::Escape => {
            *control_flow = ControlFlow::Exit;
          }
          _ => {}
        },
        _ => {}
      }
    }
    Event::NewEvents(StartCause::Init) => {
      *control_flow = ControlFlow::WaitUntil(Instant::now() + period)
    }
    Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
      *control_flow = ControlFlow::WaitUntil(Instant::now() + period);
      field.make_step();
      window.request_redraw();
    }
    Event::RedrawRequested(_) => {
      field.draw(pixels.get_frame());
      if pixels.render().is_err() {
        *control_flow = ControlFlow::Exit;
      }
    }
    _ => {}
  });
}
