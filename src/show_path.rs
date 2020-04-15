use crate::navigation::{Location, Path};
use crate::office::Office;

pub fn show_path(office: &Office, path: &Path) -> String {
    let locations: Vec<&Location> = path.iter().map(|(location, _)| location).collect();

    let office_as_string: String = office.to_string();
    let mut row_strings: Vec<String> = office_as_string
        .split_whitespace()
        .map(|str| str.to_string())
        .collect();

    for location in locations {
        let row_string = row_strings.get_mut(location.row).unwrap();
        row_string.replace_range(location.col..location.col + 1, "-");
    }

    row_strings.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::navigation::{Navigator, WallHuggingNavigator};
    use crate::office::office_from_string;
    #[test]
    fn it_can_print_an_office_and_path() {
        let office_string = "\
xx0000000
000xxx000
0x000x000
0x0x0xxx0
000x000x0"
            .to_string();
        let office = office_from_string(office_string);
        let mut navigator = WallHuggingNavigator {};

        let path = navigator.try_to_escape(&office, 6).unwrap();

        assert_eq!(
            show_path(&office, &path),
            "\
xx-000000
---xxx000
-x---x000
-x-x-xxx0
---x---x0"
        )
    }
}
