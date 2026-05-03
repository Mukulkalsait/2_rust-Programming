// use std::future::{self, Future};

use futures::join;

fn main() {
    let num1 = get_number_async();
    let num2 = num2();
    let num3 = num3();

    let res = smol::block_on(async { futures::join!(num1, num2, num3) }); // keep pooling
    let res2 = smol::block_on(async {
        futures::select! {}
    });

    println!("{:?}", res);
}

async fn get_number_async() -> u8 {
    println!("Running");
    8
}

async fn num2() -> u8 {
    std::thread::sleep(std::time::Duration::from_secs(2));
    23
}

async fn num3() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(40));
    39
}

// Y: another way to do
// fn another_way_to_async() -> impl Future<u8> {}
