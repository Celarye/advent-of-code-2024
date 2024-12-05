mod _1;
mod _2;
mod _3;
mod _4;

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

    let _3_results;
    match _3::init() {
        Ok(_3) => _3_results = _3.results(),
        Err(()) => _3_results = vec!["Error".to_string(), "Error".to_string()],
    }

    let _4_results;
    match _4::init() {
        Ok(_4) => _4_results = _4.results(),
        Err(()) => _4_results = vec!["Error".to_string(), "Error".to_string()],
    }

    vec![_1_results, _2_results, _3_results, _4_results]
}
