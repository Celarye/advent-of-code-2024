mod _1;
mod _2;

pub fn results() -> Vec<Vec<String>> {
    let _1_results;
    match _1::init() {
        Ok(_1) => _1_results = _1.results(),
        Err(()) => _1_results = vec!["Error".to_string(), "Error".to_string()],
    }

    let _2_results;
    match _2::init() {
        Ok(_2) => _2_results = _2.results(),
        Err(()) => _2_results = vec!["Error".to_string(), "Error".to_string()],
    }

    vec![_1_results, _2_results]
}
