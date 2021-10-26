use rand::{
  distributions::{Distribution, Standard},
  Rng,
};
use std::collections::{HashMap, VecDeque};

pub const FIELD_WIDTH: usize = 10;
pub const FIELD_HEIGHT: usize = 20;

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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Color {
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

impl From<FigureKind> for (Pos, HashMap<Rotation, [[u8; 4]; 4]>) {
  fn from(fig: FigureKind) -> (Pos, HashMap<Rotation, [[u8; 4]; 4]>) {
    #[rustfmt::skip]
        let t = match fig {
      FigureKind::Bar => (
        Pos { x: 4, y: -2 },
        [
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
      ),
      FigureKind::PZ => (
        Pos { x: 4, y: 0 },
        [
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
      ),
      FigureKind::NZ => (
        Pos { x: 4, y: 0 },
        [
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
      ),
      FigureKind::PL => (
        Pos { x: 4, y: -1 },
        [
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
      ),
      FigureKind::NL => (
        Pos { x: 4, y: -1 },
        [
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
      ),
      FigureKind::Square => (
        Pos { x: 4, y: 0 },
        [
          (
            Rotation::None,
            [[1, 1, 0, 0],
              [1, 1, 0, 0],
              [0, 0, 0, 0],
              [0, 0, 0, 0]],
          ),
          (
            Rotation::Left,
            [[1, 1, 0, 0],
              [1, 1, 0, 0],
              [0, 0, 0, 0],
              [0, 0, 0, 0]],
          ),
          (
            Rotation::Right,
            [[1, 1, 0, 0],
              [1, 1, 0, 0],
              [0, 0, 0, 0],
              [0, 0, 0, 0]],
          ),
          (
            Rotation::Upside,
            [[1, 1, 0, 0],
              [1, 1, 0, 0],
              [0, 0, 0, 0],
              [0, 0, 0, 0]],
          ),
        ],
      ),
      FigureKind::T => (
        Pos { x: 4, y: -1 },
        [
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
      ),
    };
    (t.0, HashMap::<_, _>::from_iter(t.1))
  }
}
impl FigureKind {
  fn get_rect(self, rotation: Rotation) -> [[u8; 4]; 4] {
    let (_, t) = self.into();
    t.get(&rotation).unwrap().clone()
  }
  fn get_pos(self) -> Pos {
    let (t, _) = self.into();
    t
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
pub struct Field {
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
  pub fn new() -> Self {
    let width = FIELD_WIDTH;
    let height = FIELD_HEIGHT;
    let v = vec![vec![Color::Transparent; width]; height];
    let kind: FigureKind = rand::thread_rng().gen();
    Field {
      width,
      height,
      pieces: v,
      current_figure: Figure {
        kind,
        color: rand::thread_rng().gen(),
      },
      current_figure_pos: kind.get_pos(),
      current_figure_rotation: Rotation::None,
      state: FieldState::Playing,
      score: 0,
    }
  }
  fn place_current_figure(&mut self) {
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
          if (0..self.width).contains(&fx)
            && (0..self.height).contains(&fy)
          {
            self.pieces[fy][fx] = self.current_figure.color;
          }
        }
      }
    }
  }
  pub fn process_input(&mut self, input: InputField) {
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
  pub fn make_step(&mut self) {
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
    self.place_current_figure();
    self.check_lines();
    self.current_figure_rotation = Rotation::None;
    self.current_figure = Figure {
      kind: rand::thread_rng().gen(),
      color: rand::thread_rng().gen(),
    };
    self.current_figure_pos = self.current_figure.kind.get_pos();
  }
  fn check_lines(&mut self) {
    if let Some((y, _)) = self
      .pieces
      .iter()
      .enumerate()
      .find(|(_, line)| line.iter().all(|x| x != &Color::Transparent))
    {
      self.pieces.remove(y);
      self.pieces.insert(0, vec![Color::Transparent; self.width]);
      self.check_lines()
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
pub enum InputField {
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

impl Field {
  pub fn draw_array(
    &self,
  ) -> [Color; FIELD_HEIGHT * (FIELD_WIDTH + 4 * 2)] {
    let mut result =
      [Color::Transparent; FIELD_HEIGHT * (FIELD_WIDTH + 4 * 2)];

    for (i, pixel) in result.iter_mut().enumerate() {
      let x = i % (FIELD_WIDTH + 4 * 2);
      let y = i / (FIELD_WIDTH + 4 * 2);

      if x < 4 {
      } else if x < FIELD_WIDTH + 4 {
        let id_x = x - 4;
        let id_y = y;
        let mut color = self.pieces[id_y][id_x];

        if (self.current_figure_pos.x..self.current_figure_pos.x + 4)
          .contains(&(id_x as isize))
          && (self.current_figure_pos.y..self.current_figure_pos.y + 4)
            .contains(&(id_y as isize))
        {
          let id_x = (id_x as isize - self.current_figure_pos.x) as usize;
          let id_y = (id_y as isize - self.current_figure_pos.y) as usize;

          if {
            self.current_figure.get_rect(self.current_figure_rotation)
              [id_y][id_x]
              == 1
          } {
            color = self.current_figure.color.into();
          }
        }
        *pixel = color;
      } else {
      }
    }
    result
  }
}
