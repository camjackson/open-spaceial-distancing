use rand::seq::SliceRandom;
use std::fmt;
use std::iter;

#[derive(Clone)]
pub struct Desk {
    pub is_occupied: bool,
}

pub struct Office {
    pub desk_rows: Vec<Vec<Desk>>,
}

impl Office {
    pub fn new(width: usize, height: usize, p: f32) -> Office {
        let mut rng = rand::thread_rng();
        let number_of_occupied_desks = ((width * height) as f32 * p) as usize;

        let occupied_desks: Vec<Desk> = iter::repeat(Desk { is_occupied: true })
            .take(number_of_occupied_desks)
            .collect();
        let unoccupied_desks: Vec<Desk> = iter::repeat(Desk { is_occupied: false })
            .take(width * height - number_of_occupied_desks)
            .collect();
        let mut desk_array = [occupied_desks, unoccupied_desks].concat();
        desk_array.shuffle(&mut rng);

        let desk_rows: Vec<Vec<Desk>> = desk_array
            .chunks(width)
            .map(|chunk| chunk.to_vec())
            .collect();

        Office { desk_rows }
    }

    pub fn desk_is_occupied(&self, row: usize, col: usize) -> bool {
        unsafe {
            self.desk_rows
                .get_unchecked(row)
                .get_unchecked(col)
                .is_occupied
        }
    }
}

impl fmt::Display for Desk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_occupied {
            write!(f, "x")
        } else {
            write!(f, "0")
        }
    }
}

impl fmt::Display for Office {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.desk_rows {
            for desk in row {
                desk.fmt(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[allow(dead_code)] // This isn't really dead, it's a util for tests
pub fn office_from_string(input: String) -> Office {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn get_number_of_occupied_desks(office: &Office) -> u32 {
        // Test util function
        let mut count = 0;

        for row in &office.desk_rows {
            for desk in row {
                if desk.is_occupied {
                    count += 1;
                }
            }
        }
        count
    }

    #[test]
    fn it_can_create_an_empty_office() {
        let office = Office::new(2, 2, 0.0);
        for row in office.desk_rows {
            for desk in row {
                assert_eq!(desk.is_occupied, false);
            }
        }
    }
    #[test]
    fn it_can_create_a_full_office() {
        let office = Office::new(2, 2, 1.0);
        for row in office.desk_rows {
            for desk in row {
                assert_eq!(desk.is_occupied, true);
            }
        }
    }

    #[test]
    fn it_can_create_offices_of_varying_occupancy() {
        let mut office;
        office = Office::new(3, 4, 0.5);
        assert_eq!(get_number_of_occupied_desks(&office), 6);

        office = Office::new(10, 10, 0.9);
        assert_eq!(get_number_of_occupied_desks(&office), 90);

        office = Office::new(100, 50, 0.6);
        assert_eq!(get_number_of_occupied_desks(&office), 3000);
    }
    #[test]
    fn it_can_format_itself() {
        let desk_rows = vec![
            vec![
                Desk { is_occupied: false },
                Desk { is_occupied: false },
                Desk { is_occupied: true },
            ],
            vec![
                Desk { is_occupied: true },
                Desk { is_occupied: false },
                Desk { is_occupied: true },
            ],
            vec![
                Desk { is_occupied: false },
                Desk { is_occupied: true },
                Desk { is_occupied: false },
            ],
        ];
        let office = Office { desk_rows };

        assert_eq!(
            office.to_string(),
            "\
00x
x0x
0x0
"
        );
    }
}
