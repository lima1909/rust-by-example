use futures::executor::block_on;
use futures::future::join;
use std::time;
use async_std::task;

fn main() {
    // simple async print hello
    block_on(print("hello"));
    

    // simple join fro two async calls
    let now = time::Instant::now();
    block_on(async {
        let result = join(say_hello("foo"), say_hello("bar"));
        println!("{:?}", result.await);
    });
    println!("elapsed time: {:?}", now.elapsed());
}

async fn print(s: &str) {
    println!("{}", s)
}

async fn say_hello(s: &str) -> String {
    let sleep_duration = time::Duration::from_secs(1);
    // Note we are using async-std's `task::sleep` here, not
    // thread::sleep. We must not block the thread!
    task::sleep(sleep_duration).await;
    s.to_string()
}
