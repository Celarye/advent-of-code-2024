use crate::utils;

pub struct Template {}

pub fn init() -> Result<Template, ()> {
    let mut template = Template {};

    match utils::http::request(0, None) {
        Ok(_) => (),
        Err(()) => {
            return Err(());
        }
    }
    Ok(template)
}

impl Template {
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
