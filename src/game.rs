use cortex_m_semihosting::hprintln;

pub struct Game {
    board: [[bool; 7]; 7],
    next: [[bool; 7]; 7],
    cursor_pos: (usize, usize),
    show_cursor: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [[false; 7]; 7],
            next: [[false; 7]; 7],
            cursor_pos: (0, 0),
            show_cursor: true,
        }
    }

    pub fn tick(&mut self) {
        for x in 0..5 {
            for y in 0..5 {
                let mut neighbour_count = self.neighbour_count(y+1, x+1);
                let is_alive = self.board[x as usize+1][y as usize+1];
                if is_alive {
                    neighbour_count -= 1;
                    if neighbour_count < 2 {
                        self.next[x as usize+1][y as usize+1] = false;
                    } else if neighbour_count == 3 || neighbour_count == 2 {
                        self.next[x as usize+1][y as usize+1] = true;
                    } else {
                        self.next[x as usize+1][y as usize+1] = false;
                    }
                } else {
                    if neighbour_count == 3 {
                        self.next[x as usize+1][y as usize+1] = true;
                    }
                }

            }
        }
        self.board = self.next;
        self.show_cursor = false;
    }

    pub fn place(&mut self) {
        self.board[self.cursor_pos.0+1][self.cursor_pos.1+1] = true;
    }

    fn neighbour_count(&self, row: u8, col: u8) -> u8 {
        let mut count = 0;
        for dx in -1..=1 {
            //hprintln!("{}", dx);
            for dy in -1..=1 {
                count +=
                    self.board[(col as isize + dx) as usize][(row as isize + dy) as usize] as u8;
            }
        }

        // dont count cell in the middle
        //count -= self.board[row as usize][col as usize] as u8;

        count
    }

    pub fn draw(&mut self) -> [[u8; 5]; 5] {
        let mut ret = [[0; 5]; 5];

        for x in 0..5 {
            for y in 0..5 {
                ret[x][y] = self.board[x + 1][y + 1] as u8;
            }
        }

        if self.show_cursor {
            ret[self.cursor_pos.0][self.cursor_pos.1] = 1;
        }

        ret
    }

    pub fn cursor_up(&mut self) {
        self.show_cursor = true;

        self.cursor_pos.0 += 1;
        self.cursor_pos.0 &= 0b111;
    }

    pub fn cursor_down(&mut self) {
        self.show_cursor = true;

        if self.cursor_pos.0 == 0 {
            return;
        }

        self.cursor_pos.0 -= 1;
    }

    pub fn cursor_left(&mut self) {
        self.show_cursor = true;

        if self.cursor_pos.1 == 0 {
            return;
        }

        self.cursor_pos.1 -= 1;
    }

    pub fn cursor_right(&mut self) {
        self.show_cursor = true;

        self.cursor_pos.1 += 1;
        self.cursor_pos.1 &= 0b111;
    }
}
