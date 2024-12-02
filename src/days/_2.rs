use crate::utils;

pub struct _2 {}

pub fn init() -> Result<_2, ()> {
    let mut _2 = _2 {};

    match utils::http::request(2, None) {
        Ok(_) => (),
        Err(()) => {
            return Err(());
        }
    }
    Ok(_2)
}

impl _2 {
    fn part1(&self) -> String {
        "No Result".to_string()
    }

    fn part2(&self) -> String {
        "No Result".to_string()
    }

    pub fn results(self) -> Vec<String> {
        vec![self.part1(), self.part2()]
    }
}
