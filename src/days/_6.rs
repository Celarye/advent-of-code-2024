use std::collections::HashMap;

use crate::utils;

pub struct _6 {
    matrix: Vec<Vec<char>>,
    current_location: Vec<usize>,
    direction_change_locations: HashMap<Vec<usize>, Vec<i16>>,
}

pub fn init() -> Result<_6, ()> {
    let mut _6 = _6 {
        matrix: vec![],
        current_location: vec![],
        direction_change_locations: HashMap::new(),
    };

    match utils::http::request(6) {
        Ok(input_string) => _6.parse_input(&input_string),
        Err(()) => Err(()),
    }
}

impl _6 {
    fn parse_input(mut self, string: &String) -> Result<_6, ()> {
        self.matrix.push(
            (0..(string.split("\n").next().unwrap().len() + 2))
                .map(|_| '+')
                .collect(),
        );

        for (y_index, string_line) in string.split("\n").enumerate() {
            if string_line.len() == 0 {
                continue;
            }

            let mut matrix_line = vec!['+'];

            for (x_index, char) in string_line.chars().enumerate() {
                if char == '^' {
                    self.current_location = vec![y_index + 1, x_index + 1];
                }

                matrix_line.push(char);
            }

            matrix_line.push('+');

            self.matrix.push(matrix_line);
        }

        self.matrix.push(
            (0..(string.split("\n").next().unwrap().len() + 2))
                .map(|_| '+')
                .collect(),
        );

        for row in self.matrix.iter() {
            for char in row.iter() {
                print!("{}", char);
            }

            print!("\n")
        }

        Ok(self)
    }

    fn part1(&mut self) -> String {
        match self.try_path() {
            Some(result) => result.to_string(),
            None => panic!("the initial try should always resolve to a result"),
        }
    }

    fn part2(&mut self) -> String {
        // let mut result = 0;

        // TODO: Fix index bug in the try_path method and implement a decent solution for part 2.

        // Brute force:

        // match self.try_path() {
        //     Some(_) => (),
        //     None => panic!("the initial try should always resolve to a result"),
        // }

        // for (index_y, row) in self.matrix.clone().iter().enumerate() {
        //     for (index_x, char) in row.iter().enumerate() {
        //         if char == &'.' {
        //             self.matrix[index_y][index_x] = '#';

        //             match self.try_path() {
        //                 Some(_) => (),
        //                 None => result += 1,
        //             }

        //             self.matrix[index_y][index_x] = *char;
        //         }
        //     }
        // // }

        // More performant solution:

        // match self.try_path() {
        //     Some(_) => (),
        //     None => panic!("the initial try should always resolve to a result"),
        // }

        // for (direction_change_location, direction_change_metadata) in
        //     self.direction_change_locations.clone()
        // {
        //     println!(
        //         "Test: [{}, {}] | {} | {}",
        //         direction_change_location[0] + 1,
        //         direction_change_location[1] + 1,
        //         self.matrix[direction_change_location[0]][direction_change_location[1]],
        //         result
        //     );

        //     let mut relative_check_location =
        //         vec![direction_change_metadata[1], direction_change_metadata[2]];

        //     loop {
        //         println!(
        //             "Found: [{}, {}] | {}",
        //             (direction_change_location[0] as i16 + relative_check_location[0] + 1) as usize,
        //             (direction_change_location[1] as i16 + relative_check_location[1] + 1) as usize,
        //             self.matrix[(direction_change_location[0] as i16 + relative_check_location[0])
        //                 as usize][(direction_change_location[1] as i16
        //                 + relative_check_location[1]) as usize]
        //         );

        //         if self.matrix
        //             [(direction_change_location[0] as i16 + relative_check_location[0]) as usize]
        //             [(direction_change_location[1] as i16 + relative_check_location[1]) as usize]
        //             == 'X'
        //         {
        //             let original_value = self.matrix[(direction_change_location[0] as i16
        //                 + relative_check_location[0]
        //                 + direction_change_metadata[1])
        //                 as usize][(direction_change_location[1] as i16
        //                 + relative_check_location[1]
        //                 + direction_change_metadata[2]) as usize];

        //             self.matrix[(direction_change_location[0] as i16
        //                 + relative_check_location[0]
        //                 + direction_change_metadata[1]) as usize]
        //                 [(direction_change_location[1] as i16
        //                     + relative_check_location[1]
        //                     + direction_change_metadata[2]) as usize] = '#';

        //             println!(
        //                 "Set: [{}, {}] | {}",
        //                 (direction_change_location[0] as i16
        //                     + relative_check_location[0]
        //                     + direction_change_metadata[1]) as usize,
        //                 (direction_change_location[1] as i16
        //                     + relative_check_location[1]
        //                     + direction_change_metadata[2]) as usize,
        //                 self.matrix[(direction_change_location[0] as i16
        //                     + relative_check_location[0]
        //                     + direction_change_metadata[1])
        //                     as usize][(direction_change_location[1] as i16
        //                     + relative_check_location[1]
        //                     + direction_change_metadata[2])
        //                     as usize]
        //             );

        //             match self.try_path() {
        //                 Some(_) => {
        //                     self.matrix[(direction_change_location[0] as i16
        //                         + relative_check_location[0]
        //                         + direction_change_metadata[1])
        //                         as usize][(direction_change_location[1]
        //                         as i16
        //                         + relative_check_location[1]
        //                         + direction_change_metadata[2])
        //                         as usize] = original_value;

        //                     relative_check_location = vec![
        //                         relative_check_location[0] + direction_change_metadata[1],
        //                         relative_check_location[1] + direction_change_metadata[2],
        //                     ];

        //                     continue;
        //                 }
        //                 None => {
        //                     result += 1;

        //                     self.matrix[(direction_change_location[0] as i16
        //                         + relative_check_location[0]
        //                         + direction_change_metadata[1])
        //                         as usize][(direction_change_location[1]
        //                         as i16
        //                         + relative_check_location[1]
        //                         + direction_change_metadata[2])
        //                         as usize] = original_value;

        //                     break;
        //                 }
        //             }
        //         } else if self.matrix
        //             [(direction_change_location[0] as i16 + relative_check_location[0]) as usize]
        //             [(direction_change_location[1] as i16 + relative_check_location[1]) as usize]
        //             == '#'
        //             || self.matrix[(direction_change_location[0] as i16
        //                 + relative_check_location[0]) as usize]
        //                 [(direction_change_location[1] as i16 + relative_check_location[1])
        //                     as usize]
        //                 == '+'
        //         {
        //             break;
        //         } else {
        //             relative_check_location = vec![
        //                 relative_check_location[0] + direction_change_metadata[1],
        //                 relative_check_location[1] + direction_change_metadata[2],
        //             ];
        //         }
        //     }
        // }

        // result.to_string()

        "No Result".to_string()
    }

    fn try_path(&mut self) -> Option<u16> {
        let mut result = 1;

        let mut direction: Vec<i16> = vec![-1, 0];

        loop {
            self.matrix[self.current_location[0]][self.current_location[1]] = 'X';

            match self.matrix[(self.current_location[0] as i16 + direction[0]) as usize]
                [(self.current_location[1] as i16 + direction[1]) as usize]
            {
                '+' => break,
                '#' => {
                    let direction_change = match self.direction_change_locations.get(&vec![
                        (self.current_location[0] as i16 + direction[0]) as usize,
                        (self.current_location[1] as i16 + direction[1]) as usize,
                    ]) {
                        Some(metadata) => metadata,
                        None => &vec![0, 0, 0],
                    };

                    if direction_change[0] == 3 {
                        return None;
                    } else {
                        self.direction_change_locations.insert(
                            vec![
                                (self.current_location[0] as i16 + direction[0]) as usize,
                                (self.current_location[1] as i16 + direction[1]) as usize,
                            ],
                            vec![direction_change[0] + 1, -direction[0], -direction[1]],
                        );
                    }

                    match direction[..] {
                        [-1, 0] => direction = vec![0, 1],
                        [0, 1] => direction = vec![1, 0],
                        [1, 0] => direction = vec![0, -1],
                        _ => direction = vec![-1, 0],
                    }
                }
                _ => (),
            }

            self.current_location = vec![
                (self.current_location[0] as i16 + direction[0]) as usize,
                (self.current_location[1] as i16 + direction[1]) as usize,
            ];

            if self.matrix[self.current_location[0]][self.current_location[1]] == '.' {
                result += 1
            }
        }

        Some(result)
    }

    pub fn results(mut self) -> Vec<String> {
        vec![self.part1(), self.part2()]
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use super::_6;

    pub fn test_init() -> Result<_6, ()> {
        let mut _6 = _6 {
            matrix: vec![],
            current_location: vec![],
            direction_change_locations: HashMap::new(),
        };

        match fs::read_to_string("input_examples/day6.txt") {
            Ok(input_string) => _6.parse_input(&input_string),
            Err(err) => {
                eprintln!(
                    "An error occurred while loading the example input: {}",
                    &err
                );
                Err(())
            }
        }
    }

    #[test]
    fn part1_test() {
        let mut _6 = test_init().unwrap();
        assert_eq!(41.to_string(), _6.part1())
    }

    #[test]
    fn part2_test() {
        let mut _6 = test_init().unwrap();
        assert_eq!(6.to_string(), _6.part2())
    }
}
