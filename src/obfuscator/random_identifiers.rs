use std::char;

use rand::{distributions::DistString, Rng};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_string_is_valid_python_identifier() {
        for _ in 0..100 {
            let result = rand_str();

            assert_eq!(result.starts_with(char::is_alphabetic), true);
            assert_eq!(result.chars().all(char::is_alphanumeric), true);
        }
    }
}

pub fn rand_str() -> String {
    let mut a = rand::distributions::Alphanumeric.sample_string(
        &mut rand::thread_rng(),
        rand::thread_rng().gen_range(10..30),
    );
    if a.starts_with(char::is_numeric) {
        a.insert(0, rand::thread_rng().gen_range(b'A'..=b'Z') as char);
    }
    a
}
