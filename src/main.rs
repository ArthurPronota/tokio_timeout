use tokio::time::{timeout, sleep, Duration} ;

async fn fetch_data() ->u64 {
    sleep(Duration::from_millis(10)).await;
    60
}

#[tokio::main]
async fn main() {
    match timeout(
            Duration::from_millis(5), 
            fetch_data()
        ).await {
            Ok(res) => {
                println!("Result: {res}") ;
            },
            Err(err) => {
                eprintln!("Error: {err}") ;
            },
    }
}
