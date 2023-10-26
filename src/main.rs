use raylib::prelude::*;


#[derive(Clone)]
enum Token {
  ChangeColor,
  MoveForward,
  RotateRight,
  LoopBegin,
  LoopEnd
}

impl Token {
  fn tokenization(source: &str) -> Vec<Self> {
    let mut tokens: Vec<Self> = Vec::new();

    for char in source.to_uppercase().chars() {
      let token: Option<Self> = match char {
        'C' => Some(Self::ChangeColor),
        'F' => Some(Self::MoveForward),
        'R' => Some(Self::RotateRight),
        '[' => Some(Self::LoopBegin),
        ']' => Some(Self::LoopEnd),
        _ => None
      };

      if let Some(token) = token {
        tokens.push(token);
      }
    }

    tokens
  }
}

#[derive(Clone, Copy)]
enum Direction {
  North = 0,
  Northeast = 1,
  East = 2,
  Southeast = 3,
  South = 4,
  Southwest = 5,
  West = 6,
  Northwest = 7
}

impl Direction {
  const DIRECTION_COUNT: u8 = 8;
}

impl From<u8> for Direction {
  fn from(value: u8) -> Self {
    match value {
      0 => Self::North,
      1 => Self::Northeast,
      2 => Self::East,
      3 => Self::Southeast,
      4 => Self::South,
      5 => Self::Southwest,
      6 => Self::West,
      7 => Self::Northwest,
      _ => Self::North
    }
  }
}

enum Command {
  ChangeColor,
  MoveForward,
  RotateRight,
  Loop(Vec<Command>)
}

impl Command {
  fn parse_command(tokens: Vec<Token>) -> Option<Vec<Self>> {
    let mut commands: Vec<Self> = Vec::new();
    let mut loop_depth: i32 = 0;
    let mut loop_start: usize = 0;

    for (index, token) in tokens.iter().enumerate() {
      if loop_depth == 0 {
        let command: Option<Command> = match token {
          Token::ChangeColor => Some(Self::ChangeColor),
          Token::MoveForward => Some(Self::MoveForward),
          Token::RotateRight => Some(Self::RotateRight),
          Token::LoopBegin => {
            loop_depth += 1;
            loop_start = index;
            None
          },
          Token::LoopEnd => return None
        };
        
        if let Some(command) = command {
          commands.push(command);
        }
      } else {
        match token {
          Token::LoopBegin => {
            loop_depth += 1;
          }
          Token::LoopEnd => {
            loop_depth -= 1;

            if loop_depth == 0 {
              commands.push(
                Self::Loop(
                  Self::parse_command(
                    tokens[(loop_start + 1)..index].to_vec()
                  )?
                )
              )
            }
          }
          _ => ()
        };
      }
    }

    if loop_depth != 0 {
      return None;
    }

    Some(commands)
  }
}


struct Machine {
  location: (u8, u8),
  heading: Direction,
  color_index: usize
}

impl Machine {
  const COLORS: [&'static str; 8] = [
    "000000", "3366ff", "00cc00", "00cccc", "cc0000", "cc00cc", "cccc00", "cccccc"
  ];

  fn draw_canva(&mut self, d: &mut RaylibDrawHandle, commands: &Vec<Command>) {
    commands.into_iter().for_each(
      |command: &Command| {
        match command {
          Command::ChangeColor => {
            self.color_index = (self.color_index + 1) % Machine::COLORS.len();
          },
          Command::MoveForward => {
            match self.heading {
              Direction::North => {
                self.location.1 = self.location.1.wrapping_sub(1);
              },
              Direction::Northeast => {
                self.location.0 = self.location.0.wrapping_add(1);
                self.location.1 = self.location.1.wrapping_sub(1);
              },
              Direction::East => {
                self.location.0 = self.location.0.wrapping_add(1);
              },
              Direction::Southeast => {
                self.location.0 = self.location.0.wrapping_add(1);
                self.location.1 = self.location.1.wrapping_add(1);
              },
              Direction::South => {
                self.location.1 = self.location.1.wrapping_add(1);
              },
              Direction::Southwest => {
                self.location.0 = self.location.0.wrapping_sub(1);
                self.location.1 = self.location.1.wrapping_add(1);
              },
              Direction::West => {
                self.location.0 = self.location.0.wrapping_sub(1);
              },
              Direction::Northwest => {
                self.location.0 = self.location.0.wrapping_sub(1);
                self.location.1 = self.location.1.wrapping_sub(1);
              }
            }

            d.draw_rectangle(
              self.location.0 as i32 * 4,
              self.location.1 as i32 * 4,
              4,
              4,
              Color::from_hex(
                Machine::COLORS[self.color_index]
              ).expect("Failed to parse color")
            );
          },
          Command::RotateRight => {
            self.heading = Direction::from((self.heading as u8 + 1) % Direction::DIRECTION_COUNT);
          },
          Command::Loop(loop_commands) => {
            (0..2).for_each(|_| self.draw_canva(d, loop_commands))
          },
        }
      }
    );
  }

  fn run(d: &mut RaylibDrawHandle, source: &str) {
    let tokens: Vec<Token> = Token::tokenization(source);

    let commands: Option<Vec<Command>> = Command::parse_command(tokens);

    if source.len() > 256 {
      d.draw_rectangle(0, 0, 1024, 1024, Color::RED);
      d.draw_text(
        "Command max length exceeded (256 bytes)",
        44,
        500,
        44,
        Color::BLACK
      );
      return;
    }

    if let Some(commands) = commands {
      let mut machine: Machine = Machine {
        location: (126, 126),
        heading: Direction::North,
        color_index: Machine::COLORS.len() - 1
      };
  
      machine.draw_canva(d, &commands);
    } else {
      d.draw_rectangle(0, 0, 1024, 1024, Color::RED);
      d.draw_text(
        "Unclosed delimiter",
        290,
        500,
        50,
        Color::BLACK
      );
    }
  }
}


fn text_input_handling(rl: &mut RaylibHandle, input_box: Rectangle, command: &mut String, cursor_location: &mut usize) {
  if input_box.check_collision_point_rec(rl.get_mouse_position()) {
    rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_IBEAM);
  } else {
    rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
  }

  if (rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL)) && rl.is_key_pressed(KeyboardKey::KEY_C) {
    rl.set_clipboard_text(&command).expect("Failed to copy command to clipboard");
    return;
  }

  if (rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL)) && rl.is_key_pressed(KeyboardKey::KEY_V) {
    if let Ok(cb_text) = rl.get_clipboard_text() {
      cb_text.to_ascii_uppercase().chars().for_each(
        |c| {
          match c {
            'C' | 'F' | 'R' | '[' | ']' => {
              command.insert(
                *cursor_location,
                c
              );
              *cursor_location += 1;
            },
            _ => ()
          }
        }
      );
    }
    return;
  }

  if (rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT)) && rl.is_key_down(KeyboardKey::KEY_RIGHT) {
    if *cursor_location < command.len() {
      *cursor_location += 1;
    }
    return;
  }

  if (rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT)) && rl.is_key_down(KeyboardKey::KEY_LEFT) {
    if *cursor_location != 0 {
      *cursor_location -= 1;
    }
    return;
  }

  if (rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT)) && rl.is_key_down(KeyboardKey::KEY_BACKSPACE) {
    if *cursor_location == command.len() {
      command.pop();
    } else {
      command.remove(*cursor_location - 1);
    }
    if *cursor_location != 0 {
      *cursor_location -= 1;
    }
    return;
  }

  let mut key: Option<u32> = rl.get_key_pressed_number();

  while let Some(key_num) = key {
    match key_num {
      67 | 70 | 82 | 91 | 93 => {
        if *cursor_location == command.len() {
          command.push(
            char::from_u32(key_num).expect(
              "Failed to parse keyboard input"
            ).to_ascii_uppercase()
          );
        } else {
          command.insert(
            *cursor_location,
            char::from_u32(key_num).expect(
              "Failed to parse keyboard input"
            ).to_ascii_uppercase()
          );
        }
        *cursor_location += 1;
      },
      259 => {
        if *cursor_location == command.len() {
          command.pop();
        } else {
          command.remove(*cursor_location - 1);
        }
        if *cursor_location != 0 {
          *cursor_location -= 1;
        }
      }
      261 => {
        command.clear();
        *cursor_location = 0;
      }
      262 => {
        if *cursor_location < command.len() {
          *cursor_location += 1;
        }
      }
      263 => {
        if *cursor_location != 0 {
          *cursor_location -= 1;
        }
      },
      _ => ()
    }

    key = rl.get_key_pressed_number();
  }
}

fn main() {
  let (mut rl, thread) = raylib::init()
    .width(1024)
    .height(1560)
    .title("Rust CFR[] - https://susam.net/cfr.html")
    .msaa_4x()
    .build();

  rl.set_target_fps(30);

  let mut command: String = String::new();
  let mut cursor_location: usize = 0;

  let input_box: Rectangle = Rectangle {
    x: 0.0,
    y: 1024.0,
    width: 1024.0,
    height: 536.0
  };
  let text_box: Rectangle = Rectangle {
    x: 8.0,
    y: 1032.0,
    width: 1022.0,
    height: 536.0
  };

  let default_font: Font = unsafe { Font::from_raw(ffi::GetFontDefault()) };

  while !rl.window_should_close() {
    text_input_handling(&mut rl, input_box, &mut command, &mut cursor_location);

    let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
    
    d.clear_background(Color::BLACK);
    
    Machine::run(&mut d, &command);

    d.draw_rectangle_rec(input_box, Color::BLACK);
    d.draw_rectangle_lines_ex(
      input_box,
      2,
      Color::LIGHTGRAY
    );

    let mut command_with_cursor: String = command.clone();
    command_with_cursor.insert(cursor_location, '_');

    d.draw_text_rec(
      &default_font,
      &command_with_cursor,
      text_box,
      40.0,
      8.0,
      true,
      Color::WHITE
    );
  }
}
