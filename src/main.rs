use pixels::{Pixels, SurfaceTexture};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::{
  collections::{HashMap, VecDeque},
  time::{Duration, Instant},
};
use winit::{
  dpi::{PhysicalSize, Size},
  event::{
    ElementState, Event, KeyboardInput, StartCause, VirtualKeyCode,
    WindowEvent,
  },
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

const WIDTH: usize = 300;
const HEIGHT: usize = WIDTH * 2;

#[derive(Copy, Clone)]
struct Pos {
  x: isize,
  y: isize,
}
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Rotation {
  None,
  Left,
  Right,
  Upside,
}
impl Rotation {
  fn rotate(&self) -> Self {
    match self {
      Rotation::None => Rotation::Right,
      Rotation::Left => Rotation::None,
      Rotation::Right => Rotation::Upside,
      Rotation::Upside => Rotation::Left,
    }
  }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Color {
  Transparent,
  Red,
  Green,
  Blue,
  Cyan,
  Magenta,
  Yellow,
}
impl From<Color> for [u8; 4] {
  fn from(color: Color) -> [u8; 4] {
    match color {
      Color::Transparent => [0, 0, 0, !0],
      Color::Red => [!0, 0, 0, !0],
      Color::Green => [0, !0, 0, !0],
      Color::Blue => [0, 0, !0, !0],
      Color::Cyan => [0, !0, !0, !0],
      Color::Magenta => [!0, 0, !0, !0],
      Color::Yellow => [!0, !0, 0, !0],
    }
  }
}
impl Distribution<Color> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
    match rng.gen_range(1..7) {
      //0 => Color::Transparent,
      1 => Color::Red,
      2 => Color::Green,
      3 => Color::Blue,
      4 => Color::Cyan,
      5 => Color::Magenta,
      6 => Color::Yellow,
      _ => panic!(),
    }
  }
}
#[derive(Copy, Clone)]
enum FigureKind {
  Bar,
  PZ,
  NZ,
  PL,
  NL,
  Square,
  T,
}
impl Distribution<FigureKind> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FigureKind {
    match rng.gen_range(0..7) {
      0 => FigureKind::Bar,
      1 => FigureKind::PZ,
      2 => FigureKind::NZ,
      3 => FigureKind::PL,
      4 => FigureKind::NL,
      5 => FigureKind::Square,
      6 => FigureKind::T,
      _ => panic!(),
    }
  }
}

impl From<FigureKind> for HashMap<Rotation, [[u8; 4]; 4]> {
  fn from(fig: FigureKind) -> HashMap<Rotation, [[u8; 4]; 4]> {
    #[rustfmt::skip]
    let t = match fig {
      FigureKind::Bar => [
        (
          Rotation::None,
          [[0, 1, 0, 0],
           [0, 1, 0, 0],
           [0, 1, 0, 0],
           [0, 1, 0, 0]],
        ),
        (
          Rotation::Left,
          [[0, 0, 0, 0],
           [0, 0, 0, 0],
           [1, 1, 1, 1],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Right,
          [[0, 0, 0, 0],
           [1, 1, 1, 1],
           [0, 0, 0, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Upside,
          [[0, 0, 1, 0],
           [0, 0, 1, 0],
           [0, 0, 1, 0],
           [0, 0, 1, 0]],
        ),
      ],
      FigureKind::PZ => [
        (
          Rotation::None,
          [[1, 1, 0, 0],
           [0, 1, 1, 0],
           [0, 0, 0, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Left,
          [[0, 1, 0, 0],
           [1, 1, 0, 0],
           [1, 0, 0, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Right,
          [[0, 0, 1, 0],
           [0, 1, 1, 0],
           [0, 1, 0, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Upside,
          [[0, 0, 0, 0],
           [1, 1, 0, 0],
           [0, 1, 1, 0],
           [0, 0, 0, 0]],
        ),
      ],
      FigureKind::NZ => [
        (
          Rotation::None,
          [[0, 1, 1, 0],
           [1, 1, 0, 0],
           [0, 0, 0, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Left,
          [[1, 0, 0, 0],
           [1, 1, 0, 0],
           [0, 1, 0, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Right,
          [[0, 1, 0, 0],
           [0, 1, 1, 0],
           [0, 0, 1, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Upside,
          [[0, 0, 0, 0],
           [0, 1, 1, 0],
           [1, 1, 0, 0],
           [0, 0, 0, 0]],
        ),
      ],
      FigureKind::PL => [
        (
          Rotation::None,
          [[0, 1, 0, 0],
           [0, 1, 0, 0],
           [0, 1, 1, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Left,
          [[0, 0, 1, 0],
           [1, 1, 1, 0],
           [0, 0, 0, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Right,
          [[0, 0, 0, 0],
           [1, 1, 1, 0],
           [1, 0, 0, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Upside,
          [[1, 1, 0, 0],
           [0, 1, 0, 0],
           [0, 1, 0, 0],
           [0, 0, 0, 0]],
        ),
      ],
      FigureKind::NL => [
        (
          Rotation::None,
          [[0, 1, 0, 0],
           [0, 1, 0, 0],
           [1, 1, 0, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Left,
          [[0, 0, 0, 0],
           [1, 1, 1, 0],
           [0, 0, 1, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Right,
          [[1, 0, 0, 0],
           [1, 1, 1, 0],
           [0, 0, 0, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Upside,
          [[0, 1, 1, 0],
           [0, 1, 0, 0],
           [0, 1, 0, 0],
           [0, 0, 0, 0]],
        ),
      ],
      FigureKind::Square => [
        (
          Rotation::None,
          [[0, 0, 0, 0],
           [0, 1, 1, 0],
           [0, 1, 1, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Left,
          [[0, 0, 0, 0],
           [0, 1, 1, 0],
           [0, 1, 1, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Right,
          [[0, 0, 0, 0],
           [0, 1, 1, 0],
           [0, 1, 1, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Upside,
          [[0, 0, 0, 0],
           [0, 1, 1, 0],
           [0, 1, 1, 0],
           [0, 0, 0, 0]],
        ),
      ],
      FigureKind::T => [
        (
          Rotation::None,
          [[0, 0, 0, 0],
           [1, 1, 1, 0],
           [0, 1, 0, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Left,
          [[0, 1, 0, 0],
           [0, 1, 1, 0],
           [0, 1, 0, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Right,
          [[0, 1, 0, 0],
           [1, 1, 0, 0],
           [0, 1, 0, 0],
           [0, 0, 0, 0]],
        ),
        (
          Rotation::Upside,
          [[0, 1, 0, 0],
           [1, 1, 1, 0],
           [0, 0, 0, 0],
           [0, 0, 0, 0]],
        ),
      ],
    };
    HashMap::<_, _>::from_iter(t)
  }
}
impl FigureKind {
  fn get_rect(self, rotation: Rotation) -> [[u8; 4]; 4] {
    let t: HashMap<_, _> = self.into();
    t.get(&rotation).unwrap().clone()
  }
}
struct Figure {
  kind: FigureKind,
  color: Color,
}
impl Figure {
  fn get_rect(&self, rotation: Rotation) -> [[u8; 4]; 4] {
    self.kind.get_rect(rotation)
  }
}
struct Field {
  width: usize,
  height: usize,
  pieces: Vec<Vec<Color>>,
  current_figure: Figure,
  current_figure_pos: Pos,
  current_figure_rotation: Rotation,
  state: FieldState,
  score: u32,
}
impl Field {
  fn new() -> Self {
    let width = 10;
    let height = 20;
    let v = vec![vec![Color::Transparent; width]; height];
    Field {
      width,
      height,
      pieces: v,
      current_figure: Figure {
        kind: rand::thread_rng().gen(),
        color: rand::thread_rng().gen(),
      },
      current_figure_pos: Pos { x: 4, y: 0 },
      current_figure_rotation: Rotation::None,
      state: FieldState::Playing,
      score: 0,
    }
  }
  fn put_rect_at(&mut self) {
    for (y, line) in self
      .current_figure
      .kind
      .get_rect(self.current_figure_rotation)
      .iter()
      .enumerate()
    {
      for (x, b) in line.iter().enumerate() {
        if b == &1 {
          let (fx, fy) = (
            (x as isize + self.current_figure_pos.x) as usize,
            (y as isize + self.current_figure_pos.y) as usize,
          );
          self.pieces[fy][fx] = self.current_figure.color;
        }
      }
    }
  }
  fn process_input(&mut self, input: InputField) {
    match input {
      InputField::Left => {
        let old_pos = self.current_figure_pos;
        if let Some(new_pos) = self.try_rotation_replace(
          self.current_figure_rotation,
          Pos {
            x: old_pos.x - 1,
            y: old_pos.y,
          },
        ) {
          self.current_figure_pos = new_pos;
        }
      }
      InputField::Right => {
        let old_pos = self.current_figure_pos;
        if let Some(new_pos) = self.try_rotation_replace(
          self.current_figure_rotation,
          Pos {
            x: old_pos.x + 1,
            y: old_pos.y,
          },
        ) {
          self.current_figure_pos = new_pos;
        }
      }
      InputField::Rotate => {
        let new = self.current_figure_rotation.rotate();
        if let Some(new_pos) =
          self.try_rotation_replace(new, self.current_figure_pos)
        {
          self.current_figure_rotation = new;
          self.current_figure_pos = new_pos;
        }
      }
    }
  }
  fn make_step(&mut self) {
    let new_pos = Pos {
      x: self.current_figure_pos.x,
      y: self.current_figure_pos.y + 1,
    };
    if self.does_collide(self.current_figure_rotation, new_pos)
      != CollideVariant::None
    {
      self.next_figure()
    } else {
      self.current_figure_pos = new_pos
    }
  }
  fn try_rotation_replace(&self, rot: Rotation, pos: Pos) -> Option<Pos> {
    let pos = pos;
    let d = self.does_collide(rot, pos);
    match d {
      CollideVariant::None => Some(pos),
      CollideVariant::Left => self.try_rotation_replace(
        rot,
        Pos {
          x: pos.x + 1,
          ..pos
        },
      ),
      CollideVariant::Right => self.try_rotation_replace(
        rot,
        Pos {
          x: pos.x - 1,
          ..pos
        },
      ),
      CollideVariant::BottomOrPieces => None,
    }
  }
  fn does_collide(&self, rot: Rotation, pos: Pos) -> CollideVariant {
    for (_, line) in self.current_figure.get_rect(rot).iter().enumerate() {
      for (x, b) in line.iter().enumerate() {
        let real_x = x as isize + pos.x;
        if b == &1 {
          if real_x < 0 {
            return CollideVariant::Left;
          }
          if real_x >= self.width as isize {
            return CollideVariant::Right;
          }
        }
      }
    }
    for (y, line) in self.current_figure.get_rect(rot).iter().enumerate() {
      for (x, b) in line.iter().enumerate() {
        let real_x = x as isize + pos.x;
        let real_y = y as isize + pos.y;
        if b == &1 {
          if real_y >= self.height as isize {
            return CollideVariant::BottomOrPieces;
          }
          if (0..self.width).contains(&(real_x as usize))
            && (0..self.height).contains(&(real_y as usize))
            && self.pieces[real_y as usize][real_x as usize]
              != Color::Transparent
          {
            return CollideVariant::BottomOrPieces;
          }
        }
      }
    }
    CollideVariant::None
  }
  fn next_figure(&mut self) {
    self.put_rect_at();
    self.check_last_lines();
    self.current_figure_pos = Pos { x: 4, y: 0 };
    self.current_figure_rotation = Rotation::None;
    self.current_figure = Figure {
      kind: rand::thread_rng().gen(),
      color: rand::thread_rng().gen(),
    }
  }
  fn check_last_lines(&mut self) {
    if self.pieces[self.height - 1]
      .iter()
      .all(|x| x != &Color::Transparent)
    {
      for i in (1..self.height).rev() {
        for j in 0..self.width {
          self.pieces[i][j] = self.pieces[i - 1][j];
        }
      }
      self.check_last_lines();
    }
  }
}
#[derive(Copy, Clone, Eq, PartialEq)]
enum CollideVariant {
  None,
  Left,
  Right,
  BottomOrPieces,
}
enum FieldState {
  Playing,
  Paused,
  GameOver,
}
#[derive(Copy, Clone)]
enum InputField {
  Left,
  Right,
  Rotate,
}
#[derive(Copy, Clone)]
enum Input {
  InputField(InputField),
  Down,
  Pause,
  Quit,
}
struct InputQueue {
  queue: VecDeque<InputField>,
}

trait Draw {
  fn draw(&self, frame: &mut [u8]);
}

impl Draw for Field {
  fn draw(&self, frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
      let x = i % WIDTH;
      let y = i / WIDTH;

      let id_x = x / (WIDTH / self.width);
      let id_y = y / (HEIGHT / self.height);
      let mut color: [u8; 4] = self.pieces[id_y][id_x].into();

      if (self.current_figure_pos.x..self.current_figure_pos.x + 4)
        .contains(&(id_x as isize))
        && (self.current_figure_pos.y..self.current_figure_pos.y + 4)
          .contains(&(id_y as isize))
      {
        let id_x = (id_x as isize - self.current_figure_pos.x) as usize;
        let id_y = (id_y as isize - self.current_figure_pos.y) as usize;

        if {
          self.current_figure.get_rect(self.current_figure_rotation)[id_y]
            [id_x]
            == 1
        } {
          color = self.current_figure.color.into();
        }
      }
      pixel.copy_from_slice(&color);
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

  let period = Duration::new(0, 1000_000_000 / 4);

  let mut field = Field::new();

  event_loop.run(move |event, _, control_flow| match event {
    Event::WindowEvent { event, .. } => match event {
      WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
      WindowEvent::KeyboardInput {
        input:
          KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(VirtualKeyCode::Left),
            ..
          },
        ..
      } => {
        field.process_input(InputField::Left);
        window.request_redraw();
      }
      WindowEvent::KeyboardInput {
        input:
          KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(VirtualKeyCode::Right),
            ..
          },
        ..
      } => {
        field.process_input(InputField::Right);
        window.request_redraw();
      }

      WindowEvent::KeyboardInput {
        input:
          KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(VirtualKeyCode::Up),
            ..
          },
        ..
      } => {
        field.process_input(InputField::Rotate);
        window.request_redraw();
      }

      WindowEvent::KeyboardInput {
        input:
          KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(VirtualKeyCode::Space),
            ..
          },
        ..
      } => {}
      _ => {}
    },
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
      println!("draw");
      if pixels.render().is_err() {
        *control_flow = ControlFlow::Exit;
      }
    }

    _ => (),
  });
}
