use tokio::runtime::Runtime;

#[no_mangle]
pub extern "C" fn get_sum(x: i32, y: i32) -> i32 {
    x + y
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

async fn get_http_async(s: &str) -> String {
    reqwest::get(s).await.unwrap().text().await.unwrap()
}

#[no_mangle]
pub extern fn get_http() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let resp = get_http_async("https://httpbin.org/ip").await;
        println!("Response is:\n{}\n", resp);
    });
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
