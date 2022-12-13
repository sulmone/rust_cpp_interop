#[no_mangle]
pub extern "C" fn get_sum(x: i32, y: i32) -> i32 {
    x + y
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub async fn get_http() {
    let response = reqwest::get("https://httpbin.org/ip")
        .await.expect("Can't get ip");

    let ip = response.text().await.expect("Can't get IP");

    println!("IP: {}", ip);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        get_http();
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
