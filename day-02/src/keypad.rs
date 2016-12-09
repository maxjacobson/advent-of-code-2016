use button::Button;

#[derive(Debug)]
enum KeypadDirective {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
pub struct Keypad {
    buttons: [[Button; 3]; 3],
    position: (usize, usize),
}

impl Keypad {
    pub fn new(position: (usize, usize)) -> Keypad {
        Keypad {
            buttons: [[Button::new('1'), Button::new('2'), Button::new('3')],
                      [Button::new('4'), Button::new('5'), Button::new('6')],
                      [Button::new('7'), Button::new('8'), Button::new('9')]],
            position: position, // (0, 0) is top left; (2, 2) is bottom right
        }
    }

    pub fn adjust(&self, directive: char) -> Keypad {
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
        self.buttons[self.position.0][self.position.1].val
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
