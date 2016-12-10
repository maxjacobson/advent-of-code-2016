#[derive(Debug)]
enum KeypadDirective {
    Up,
    Right,
    Down,
    Left,
}

type Button = char;

#[derive(Debug)]
pub struct Keypad {
    buttons: [[Button; 3]; 3],
    position: (usize, usize),
}

impl Keypad {
    pub fn new(position: (usize, usize)) -> Keypad {
        Keypad {
            buttons: [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']],
            position: position, // (0, 0) is top left; (2, 2) is bottom right
        }
    }

    pub fn adjust(&self, directive: Button) -> Keypad {
        let directive = self.keypad_directive_from(directive);
        let new_position = match directive {
            KeypadDirective::Up => {
                match self.position {
                    (0, _) => self.position,
                    (_, _) => (self.position.0 - 1, self.position.1),
                }
            }
            KeypadDirective::Right => {
                match self.position {
                    (_, 2) => self.position,
                    (_, _) => (self.position.0, self.position.1 + 1),
                }
            }
            KeypadDirective::Down => {
                match self.position {
                    (2, _) => self.position,
                    (_, _) => (self.position.0 + 1, self.position.1),
                }
            }
            KeypadDirective::Left => {
                match self.position {
                    (_, 0) => self.position,
                    (_, _) => (self.position.0, self.position.1 - 1),
                }
            }
        };

        Keypad::new(new_position)
    }

    pub fn current_button(&self) -> char {
        self.buttons[self.position.0][self.position.1]
    }

    fn keypad_directive_from(&self, directive: char) -> KeypadDirective {
        match directive {
            'U' => KeypadDirective::Up,
            'R' => KeypadDirective::Right,
            'D' => KeypadDirective::Down,
            'L' => KeypadDirective::Left,
            _ => panic!("BAD INPUT"),
        }
    }
}
