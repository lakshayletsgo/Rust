pub mod parent {
    pub fn get_full_name(first: &str, last: &str) -> String {
        let full_name = format!("{0} {1}", first, last);
        full_name
    }
}

pub mod parent2 {
    pub fn get_sum(first: i32, second: i32) -> i32 {
        first + second
    }
    #[allow(dead_code)]
    fn get_divide(first: i32, second: i32) -> i32 {
        first / second
    }
}
