#[derive(Debug)]
enum KeypadDirective {
    Up,
    Right,
    Down,
    Left,
}

type Button = Option<char>;

#[derive(Debug)]
pub struct Keypad {
    buttons: [[Button; 5]; 5],
    position: (usize, usize),
}

impl Keypad {
    pub fn new(position: (usize, usize)) -> Keypad {
        Keypad {
            buttons: [[None, None, Some('1'), None, None],
                      [None, Some('2'), Some('3'), Some('4'), None],
                      [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
                      [None, Some('A'), Some('B'), Some('C'), None],
                      [None, None, Some('D'), None, None]],
            position: position,
        }
    }

    pub fn adjust(&self, directive: char) -> Keypad {
        let directive = self.keypad_directive_from(directive);
        let new_position = match directive {
            KeypadDirective::Up => {
                let new_x = match self.position.0 {
                    0 => 0,
                    n => n - 1,
                };
                (new_x, self.position.1)
            }
            KeypadDirective::Right => {
                let new_y = match self.position.1 {
                    4 => 4,
                    n => n + 1,
                };
                (self.position.0, new_y)
            }
            KeypadDirective::Down => {
                let new_x = match self.position.0 {
                    4 => 4,
                    n => n + 1,
                };
                (new_x, self.position.1)
            }
            KeypadDirective::Left => {
                let new_y = match self.position.1 {
                    0 => 0,
                    n => n - 1,
                };
                (self.position.0, new_y)
            }
        };

        let potential_new_keypad = Keypad::new(new_position);
        match potential_new_keypad.current_button() {
            Some(_) => potential_new_keypad,
            None => Keypad::new(self.position),
        }
    }

    pub fn current_button(&self) -> Button {
        self.buttons[self.position.0][self.position.1]
    }

    fn keypad_directive_from(&self, raw_directive: char) -> KeypadDirective {
        match raw_directive {
            'U' => KeypadDirective::Up,
            'R' => KeypadDirective::Right,
            'D' => KeypadDirective::Down,
            'L' => KeypadDirective::Left,
            _ => panic!("BAD INPUT"),
        }
    }
}
