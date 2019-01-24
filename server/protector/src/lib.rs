extern crate rand;

use rand::Rng;

use std::char;

// generate 10 char random string
pub fn get_session_key() -> String {
    let mut rng = rand::thread_rng();
    let mut s = String::new();

    for _ in 0..10 {
        let rand_number = rng.gen_range(1, 9);
        let c = char::from_digit(rand_number, 10).unwrap();
        s.push(c);
    }

    return s
}

// calculate initial hash string
pub fn get_hash_str() -> String {
    let mut rng = rand::thread_rng();
    let mut s = String::new();

    for _ in 0..5 {
        let rand_number = rng.gen_range(1, 7);
        let c = char::from_digit(rand_number, 10).unwrap();
        s.push(c);
    }

    return s
}

// struct used to protect web services from unauthorized access
pub struct SessionProtector {
    hash: String,
}

impl SessionProtector {
    pub fn new(hash_str: String) -> SessionProtector {
        let hash = hash_str;

        SessionProtector {
            hash,
        }
    }

    pub fn next_session_key(&self, session_key: String) -> Result<String, String> {
        if self.hash.is_empty() {
            return Err(String::from("Hash code is empty"))
        }

        for idx in 0..self.hash.len() {
            let i = self.hash.as_bytes()[idx];
            let c = char::from(i);
            if !c.is_digit(10) {
                return Err(String::from("Hash code contains non-digit letter"))
            }
        }

        let mut result: i32 = 0;

        let mut str = result.to_string();
        return Ok(str);
    }

    fn calc_hash(&self, session_key: String, val: i32) -> String {
        // TODO: translate
        let mut result = String::new();

        return result
    }
}
