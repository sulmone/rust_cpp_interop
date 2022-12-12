#[no_mangle]
pub extern "C" fn get_sum(x: i32, y: i32) -> i32 {
    x + y
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
