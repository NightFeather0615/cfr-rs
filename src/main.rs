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
  heading: usize,
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
              0 => {
                self.location.1 = self.location.1.wrapping_sub(1);
              },
              45 => {
                self.location.0 = self.location.0.wrapping_add(1);
                self.location.1 = self.location.1.wrapping_sub(1);
              },
              90 => {
                self.location.0 = self.location.0.wrapping_add(1);
              },
              135 => {
                self.location.0 = self.location.0.wrapping_add(1);
                self.location.1 = self.location.1.wrapping_add(1);
              },
              180 => {
                self.location.1 = self.location.1.wrapping_add(1);
              },
              225 => {
                self.location.0 = self.location.0.wrapping_sub(1);
                self.location.1 = self.location.1.wrapping_add(1);
              },
              270 => {
                self.location.0 = self.location.0.wrapping_sub(1);
              },
              315 => {
                self.location.0 = self.location.0.wrapping_sub(1);
                self.location.1 = self.location.1.wrapping_sub(1);
              },
              _ => ()
            }

            d.draw_rectangle(
              self.location.0 as i32 * 4,
              self.location.1 as i32 * 4,
              4,
              4,
              Color::from_hex(
                Machine::COLORS[self.color_index]
              ).unwrap()
            );
          },
          Command::RotateRight => {
            self.heading = (self.heading + 45) % 360;
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
        heading: 0,
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


fn main() {
  let (mut rl, thread) = raylib::init()
    .width(1024)
    .height(1474)
    .title("Rust CFR[] - https://susam.net/cfr.html")
    .msaa_4x()
    .build();

  rl.set_target_fps(60);

  let mut command: String = String::new();

  let input_box: Rectangle = Rectangle {
    x: 0.0,
    y: 1024.0,
    width: 1024.0,
    height: 470.0
  };
  let text_box: Rectangle = Rectangle {
    x: 8.0,
    y: 1032.0,
    width: 1024.0,
    height: 470.0
  };
  let mut mouse_on_text: bool;

  while !rl.window_should_close() {
    if input_box.check_collision_point_rec(rl.get_mouse_position()) {
      mouse_on_text = true;
    } else {
      mouse_on_text = false;
    }

    if mouse_on_text {
      rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_IBEAM);

      let mut key: Option<u32> = rl.get_key_pressed_number();

      while let Some(key_num) = key {
        if [67, 70, 82, 91, 93].contains(&key_num) {
          command.push(char::from_u32(key_num).unwrap().to_ascii_uppercase());
        }

        key = rl.get_key_pressed_number();
      }

      if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
        command.pop();
      }

      if (rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT)) && rl.is_key_down(KeyboardKey::KEY_BACKSPACE) {
        command.pop();
      }

      if rl.is_key_pressed(KeyboardKey::KEY_DELETE) {
        command.clear();
      }

      if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL) {
        if rl.is_key_pressed(KeyboardKey::KEY_V) {
          if let Ok(cb_text) = rl.get_clipboard_text() {
            for c in cb_text.to_ascii_uppercase().chars() {
              if ['C', 'F', 'R', '[', ']'].contains(&c) {
                command.push(c);
              }
            }
          }
        }
      }
    } else {
      rl.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
    }

    let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
    
    d.clear_background(Color::BLACK);

    Machine::run(&mut d, &command);

    d.draw_rectangle_rec(input_box, Color::BLACK);
    d.draw_rectangle_lines(
      input_box.x as i32,
      input_box.y as i32,
      input_box.width as i32,
      input_box.height as i32, 
      Color::LIGHTGRAY
    );
    d.draw_text_rec(
      unsafe { Font::from_raw(ffi::GetFontDefault()) },
      &command,
      text_box,
      38.0,
      8.0,
      true,
      Color::WHITE
    );
  }
}
