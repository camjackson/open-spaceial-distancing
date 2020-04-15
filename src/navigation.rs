use crate::direction::Direction;
use crate::office::Office;
use std::convert::TryInto;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Location {
    row: usize,
    col: usize,
}

type Path = Vec<(Location, Direction)>;

pub fn try_to_escape(office: &Office, start_column: usize) -> Result<Path, Path> {
    let mut current_location: Location = Location {
        row: office.desk_rows.len() - 1,
        col: start_column,
    };
    let mut current_direction: Direction = Direction::Left;
    let mut path_so_far: Path = Vec::new();

    loop {
        if current_location.row == 0 {
            // We made it out!
            return Ok(path_so_far);
        }
        if path_so_far.contains(&(current_location, current_direction)) {
            // We're in a loop
            return Err(path_so_far);
        }
        path_so_far.push((current_location, current_direction));

        let (next_location, next_direction) =
            get_next_location_and_direction(&current_location, current_direction, &office);

        current_location = next_location;
        current_direction = next_direction;
    }
}

fn get_next_location_and_direction(
    current_location: &Location,
    current_direction: Direction,
    office: &Office,
) -> (Location, Direction) {
    try_to_move_left(current_location, current_direction, office).unwrap_or_else(|| {
        try_to_move_forward(current_location, current_direction, office).unwrap_or_else(|| {
            try_to_move_right(current_location, current_direction, office).unwrap_or_else(|| {
                try_to_move_backward(current_location, current_direction, office)
                    .unwrap_or_else(|| (*current_location, current_direction))
            })
        })
    })
}

fn try_to_move_left(
    location: &Location,
    direction: Direction,
    office: &Office,
) -> Option<(Location, Direction)> {
    try_to_move_forward(location, direction.turn_left(), office)
}

fn try_to_move_right(
    location: &Location,
    direction: Direction,
    office: &Office,
) -> Option<(Location, Direction)> {
    try_to_move_forward(location, direction.turn_right(), office)
}

fn try_to_move_backward(
    location: &Location,
    direction: Direction,
    office: &Office,
) -> Option<(Location, Direction)> {
    try_to_move_forward(location, direction.turn_left().turn_left(), office)
}

fn out_of_bounds(row: i32, col: i32, width: i32, height: i32) -> bool {
    row < 0 || row >= height || col < 0 || col >= width
}

fn try_to_move_forward(
    location: &Location,
    direction: Direction,
    office: &Office,
) -> Option<(Location, Direction)> {
    // Make these signed so we can detect attempted steps into negative cells
    let current_row: i32 = location.row.try_into().unwrap();
    let current_col: i32 = location.col.try_into().unwrap();
    let office_height: i32 = office.desk_rows.len().try_into().unwrap();
    let office_width: i32 = office.desk_rows.get(0).unwrap().len().try_into().unwrap();

    let (next_row, next_col): (i32, i32) = step(current_row, current_col, direction);
    if out_of_bounds(next_row, next_col, office_width, office_height)
        || office.desk_is_occupied(next_row as usize, next_col as usize)
    {
        None
    } else {
        Some((
            Location {
                row: next_row as usize,
                col: next_col as usize,
            },
            direction,
        ))
    }
}

fn step(current_row: i32, current_col: i32, direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (current_row - 1, current_col),
        Direction::Right => (current_row, current_col + 1),
        Direction::Down => (current_row + 1, current_col),
        Direction::Left => (current_row, current_col - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::office::Desk;
    fn office_from_string(input: String) -> Office {
        let rows = input.split_whitespace();

        let desk_rows: Vec<Vec<Desk>> = rows
            .map(|row_text| {
                row_text
                    .chars()
                    .map(|character| {
                        let is_occupied = match character {
                            '0' => false,
                            'x' => true,
                            _ => panic!("Oh no!"),
                        };
                        Desk { is_occupied }
                    })
                    .collect()
            })
            .collect();
        Office { desk_rows }
    }
    #[test]
    fn it_can_navigating_all_these_offices() {
        let test_cases = [
            (
                "\
00000
00000
00000",
                2,
            ),
            (
                "\
00000
x0000
00000",
                2,
            ),
            (
                "\
00000
xxx0x
00000",
                2,
            ),
            (
                "\
00000
00000
0xx00",
                1,
            ),
            (
                "\
xx0000000
000xxx000
0x000x000
0x0x0xxx0
000x000x0",
                6,
            ),
        ];

        for (office_str, start_col) in test_cases.iter() {
            let office = office_from_string((*office_str).to_string());
            assert!(try_to_escape(&office, *start_col).is_ok());
        }
    }

    #[test]
    fn it_cannot_navigate_any_of_these_offices() {
        let test_cases = [
            (
                "\
xxxxx
xxxxx
xxxxx",
                2,
            ),
            (
                "\
00000
xxx00
x0x00",
                1,
            ),
            (
                "\
xxxxx
x0000
00000",
                2,
            ),
            (
                "\
00x00
0x0x0
x000x",
                2,
            ),
        ];
        for (office_str, start_col) in test_cases.iter() {
            let office = office_from_string((*office_str).to_string());
            assert!(!try_to_escape(&office, *start_col).is_ok());
        }
    }
}
