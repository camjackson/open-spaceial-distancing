use rand::Rng;

pub struct Desk {
    pub is_occupied: bool,
}

pub struct Office {
    pub desk_rows: Vec<Vec<Desk>>,
}

impl Office {
    pub fn new(width: usize, height: usize, p: f32) -> Office {
        let number_of_occupied_desks = ((width * height) as f32 * p) as usize;
        let occupied_indices = get_occupied_indices(width * height, number_of_occupied_desks);

        let mut desk_rows: Vec<Vec<Desk>> = Vec::with_capacity(height);
        for row_index in 0..height {
            let mut row: Vec<Desk> = Vec::with_capacity(width);
            for col_index in 0..width {
                let cell_number = row_index * width + col_index;
                let is_occupied = occupied_indices.contains(&cell_number);
                row.push(Desk { is_occupied });
            }
            desk_rows.push(row);
        }

        Office { desk_rows }
    }
}

fn get_occupied_indices(total_count: usize, occupied_count: usize) -> Vec<usize> {
    let mut list_of_all_indices: Vec<usize> = Vec::with_capacity(total_count);
    let mut list_of_occupied_indices: Vec<usize> = Vec::with_capacity(occupied_count);
    // Start with a list of all numbers from 0 to total_count
    for i in 0..total_count {
        list_of_all_indices.push(i);
    }

    // Then randomly remove elements from that list and put them in the occupied list instead
    let mut rng = rand::thread_rng();
    for _ in 0..occupied_count {
        let random_index = rng.gen_range(0, list_of_all_indices.len());
        list_of_occupied_indices.push(list_of_all_indices.remove(random_index));
    }

    list_of_occupied_indices
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
}