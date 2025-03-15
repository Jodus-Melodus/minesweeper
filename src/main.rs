use core::fmt;
use rand::Rng;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Blank,
    Bomb,
    Flag,
    Number(u8),
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blank => write!(f, "•"),
            Self::Bomb => write!(f, "B"),
            Self::Flag => write!(f, "F"),
            Self::Number(n) => write!(f, "{}", n),
        }
    }
}

fn display(field: &[[Tile; 10]; 10]) {
    println!("  0 1 2 3 4 5 6 7 8 9");
    for row in 0..10 {
        print!("{}", row);
        for column in 0..10 {
            print!(" {}", field[row][column]);
        }
        println!("");
    }
}

fn plant_mines(field: &mut [[Tile; 10]; 10]) {
    let mut rng = rand::rng();

    for row in 0..10 {
        for column in 0..10 {
            let random = rng.random_range(0..11);
            if random == 1 {
                field[row][column] = Tile::Bomb;
            }
        }
    }
}

fn populate_numbers(field: &mut [[Tile; 10]; 10]) {
    const OFFSETS: [[i8; 2]; 8] = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    let mut temp: [[Tile; 12]; 12] = [[Tile::Number(0); 12]; 12];

    for row in 0..10 {
        for column in 0..10 {
            temp[row + 1][column + 1] = field[row][column];
        }
    }

    for row in 0..12 {
        for column in 0..12 {
            if let Tile::Bomb = temp[row][column] {
                for k in 0..8 {
                    let d_row = (row as i8 + OFFSETS[k][0]) as usize;
                    let d_column = (column as i8 + OFFSETS[k][1]) as usize;

                    if let Tile::Number(ref mut c) = temp[d_row][d_column] {
                        *c += 1;
                    }
                }
            }
        }
    }

    for row in 0..10 {
        for column in 0..10 {
            field[row][column] = temp[row + 1][column + 1];
        }
    }
}

fn main() {
    let mut field: [[Tile; 10]; 10] = [[Tile::Number(0); 10]; 10];

    plant_mines(&mut field);
    populate_numbers(&mut field);
    display(&field);
}
