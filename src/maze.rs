
#[derive(Copy, Clone)]
pub enum Walls {
    Left  = 1 << 0,
    Right = 1 << 1,
    Up    = 1 << 2,
    Down  = 1 << 3,
}

impl Walls {
    fn left(v: i8) -> bool {
        v & Walls::Left as i8 != 0
    }

    fn right(v: i8) -> bool {
        v & Walls::Right as i8 != 0
    }

    fn up(v: i8) -> bool {
        v & Walls::Up as i8 != 0
    }

    fn down(v: i8) -> bool {
        v & Walls::Down as i8 != 0
    }

    pub fn opposite(&self) -> Walls {
        match *self {
            Walls::Left => Walls::Right,
            Walls::Right => Walls::Left,
            Walls::Up => Walls::Down,
            Walls::Down => Walls::Up,
        }
    }
}


pub struct Maze {
    size: usize,
    maze: Vec<Vec<i8>>,
}

impl Maze {
    pub fn new(size: usize) -> Maze {
        let mut m = vec![vec![0; size]; size];
        // Fill in edges of maze.
        for i in 0..size {
            m[i][0]        |= Walls::Left as i8;
            m[i][size - 1] |= Walls::Right as i8;
            m[0][i]        |= Walls::Up as i8;
            m[size - 1][i] |= Walls::Down as i8;
        }
        Maze { size: size, maze: m }
    }

    pub fn fill_walls(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                self.maze[i][j] =
                    Walls::Up as i8 | Walls::Down as i8 |
                    Walls::Left as i8 | Walls::Right as i8;
            }
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn print(&self) {
        // Top row
        let mut top = "╔══".to_string();
        for x in 0..(self.size - 1) {
            top.push_str(if Walls::right(self.maze[0][x]) { "╦" } else { "═" });
            top.push_str("══");
        }
        top.push('╗');
        println!("{}", top);
        // Middle rows
        for y in 0..(self.size - 1) {
            // Row
            println!("{}", self.get_cell_row(y));
            // Horizontal border
            let mut horz = if Walls::down(self.maze[y][0]) {
                    "╠══".to_string()
                } else {
                    "║  ".to_string()
                };
            for x in 1..self.size {
                horz.push(
                    Maze::get_inner_junction(
                        self.maze[y][x - 1],
                        self.maze[y + 1][x]));
                horz.push_str(if Walls::down(self.maze[y][x]) {"══"} else {"  "});
            }
            horz.push(
                if Walls::down(self.maze[y][self.size - 1]) {'╣'} else {'║'});
            println!("{}", horz);
        }
        println!("{}", self.get_cell_row(self.size - 1));
        // Final line
        let mut bot = "╚══".to_string();
        for x in 0..(self.size - 1) {
            bot.push_str(
                if Walls::right(self.maze[self.size - 1][x]) {
                    "╩"
                } else {
                    "═"
                });
            bot.push_str("══");
        }
        bot.push('╝');
        println!("{}", bot);
    }

    fn get_cell_row(&self, y: usize) -> String {
        let mut row = "║  ".to_string();
        for x in 0..(self.size - 1) {
           row.push(if Walls::right(self.maze[y][x]) {'║'} else {' '});
           row.push_str("  ");
        }
        row.push('║');
        row
    }

    fn get_inner_junction(a: i8, d: i8) -> char {
        // The inner junction is the piece connecting four squares of a maze.
        // a b
        //  ?
        // c d
        //
        // The ? can be one of ╣, ╦, ╔, ... depending on the walls between
        // each of the cells. Because every cell stores all of its walls we
        // can determine which junction to use based on the down and right
        // of 'a' and the up and left of 'd' in the diagram
        //
        // Because the cells are bit vectors and there's no overlap of the
        // bits needed from 'a' or 'd' we can combine them into a single
        // i8 and use that as a lookup. Note this will break if the values
        // in the enum change. (Is this bad practice? Feels like it).
        let lookup: i8 =
            (a & (Walls::Down as i8 | Walls::Right as i8)) |
            (d & (Walls::Up as i8 | Walls::Left as i8));
        [' ', ' ', ' ', '║', ' ', '╔', '╚', '╠',
         ' ', '╗', '╝', '╣', '═', '╦', '╩', '╬'][lookup as usize]
    }

    pub fn rm_wall(&mut self, x: usize, y: usize, w: Walls) {
        self.maze[y][x] &= !(w as i8);
    }
}

