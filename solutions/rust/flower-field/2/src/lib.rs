struct Cell {
    x: usize,
    y: usize,
}

static FLOWER: char = '*';

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return Vec::new();
    }

    (0..garden.len())
        .map(|row| {
            (0..garden[0].len())
                .map(|col| {
                    let cell = Cell { x: col, y: row };
                    annotate_cell(garden, &cell)
                })
                .collect()
        })
        .collect()
}

fn annotate_cell(garden: &[&str], cell: &Cell) -> char {
    let cell_char = garden[cell.y].chars().nth(cell.x).unwrap();
    if cell_char == FLOWER {
        return FLOWER;
    }

    let flower_count = get_neighbors(cell, garden.len(), garden[0].len())
        .iter()
        .filter(|&&(r, c)| garden[r].chars().nth(c).unwrap() == FLOWER)
        .count();

    match flower_count {
        0 => ' ',
        n => std::char::from_digit(n as u32, 10).unwrap(),
    }
}

fn get_neighbors(cell: &Cell, max_row: usize, max_col: usize) -> Vec<(usize, usize)> {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    directions
        .iter()
        .filter_map(|&(dr, dc)| {
            let new_row = cell.y as i32 + dr;
            let new_col = cell.x as i32 + dc;

            if new_row >= 0 && new_row < max_row as i32 && new_col >= 0 && new_col < max_col as i32
            {
                Some((new_row as usize, new_col as usize))
            } else {
                None
            }
        })
        .collect()
}
