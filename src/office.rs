use rand::Rng;

pub struct Desk {
    pub is_occupied: bool,
}

pub struct Office {
    pub desk_rows: Vec<Vec<Desk>>,
}

impl Office {
    pub fn new(width: usize, height: usize, p: f32) -> Office {
        let mut desk_rows: Vec<Vec<Desk>> = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row: Vec<Desk> = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Desk { is_occupied: false });
            }
            desk_rows.push(row);
        }

        let mut office = Office { desk_rows };

        let number_of_occupied_desks: u32 = ((width * height) as f32 * p) as u32;
        for _ in 0..number_of_occupied_desks {
            office.fill_a_desk_at_random();
        }
        office
    }

    fn fill_a_desk_at_random(&mut self) {
        let height = self.desk_rows.len();
        let width = self.desk_rows.get(0).unwrap().len();
        let mut rng = rand::thread_rng();

        loop {
            let row = rng.gen_range(0, height);
            let col = rng.gen_range(0, width);

            match self.try_to_fill_desk_at(row, col) {
                Err(()) => (),
                Ok(()) => break,
            }
        }
    }

    fn try_to_fill_desk_at(&mut self, row: usize, col: usize) -> Result<(), ()> {
        let desk = self.desk_rows.get_mut(row).unwrap().get_mut(col).unwrap();
        if desk.is_occupied {
            Err(())
        } else {
            desk.is_occupied = true;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_number_of_occupied_desks(office: &Office) -> u32 {
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
    fn it_can_create_a_half_full_office() {
        let office = Office::new(3, 4, 0.5);
        assert_eq!(get_number_of_occupied_desks(&office), 6);
    }

    #[test]
    fn it_can_fill_the_only_desk() {
        let mut office = Office::new(1, 1, 0.0);

        assert_eq!(get_number_of_occupied_desks(&office), 0);

        office.fill_a_desk_at_random();

        assert_eq!(get_number_of_occupied_desks(&office), 1);
    }

    #[test]
    fn it_can_fill_all_the_desks_one_by_one() {
        let width = 10;
        let height = 10;
        let mut office = Office::new(10, 10, 0.0);

        assert_eq!(get_number_of_occupied_desks(&office), 0);

        for _ in 0..(width * height) {
            office.fill_a_desk_at_random();
        }

        assert_eq!(get_number_of_occupied_desks(&office), width * height);
    }
}
