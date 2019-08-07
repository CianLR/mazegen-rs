
#[derive(Copy, Clone)]
pub enum Walls {
    Left  = 1 << 0,
    Right = 1 << 1,
    Up    = 1 << 2,
    Down  = 1 << 3,
}

pub fn wall_opposite(w: Walls) -> Walls {
    match w {
        Walls::Left => Walls::Right,
        Walls::Right => Walls::Left,
        Walls::Up => Walls::Down,
        Walls::Down => Walls::Up,
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
        println!("╔{}╗", (vec!["══".to_string(); self.size]).join("╦"));
        for y in 0..self.size {
            let mut l = String::new();
            l.push('║');
            for x in 0..self.size {
                l.push(' ');
                l.push(' ');
                if self.maze[y][x] & Walls::Right as i8 == 0 {
                    l.push(' ');
                } else {
                    l.push('║');
                }
            }
            println!("{}", l);
            let mut l2 = String::new();
            for x in 0..self.size {
                if self.maze[y][x] & Walls::Down as i8 == 0 {
                    l2.push(' ');
                    l2.push(' ');
                } else {
                    l2.push('═');
                    l2.push('═');
                }
                l2.push('╬');
            }
            println!("╠{}", l2);
        }
    }

    pub fn debug_print(&self) {
        println!("Print!");
        for line in &self.maze {
            println!("{:?}", line);
        }
    }

    pub fn add_wall(&mut self, x: usize, y: usize, w: Walls) {
        self.maze[y][x] |= w as i8;
    }

    pub fn rm_wall(&mut self, x: usize, y: usize, w: Walls) {
        self.maze[y][x] &= !(w as i8);
    }

    pub fn get_cell(&self, x: usize, y: usize) -> i8 {
        self.maze[y][x]
    }
}

