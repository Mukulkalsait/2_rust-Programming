// use std::future::{self, Future};

use futures::FutureExt;

fn main() {
    // let mut num1 = get_number_async().fuse();
    // let mut num2 = num2().fuse();
    // let mut num3 = num3().fuse();
    // let res = smol::block_on(async { futures::join!(num1, num2, num3) }); // keep pooling
    let res2 = smol::block_on(async {
        let num1 = get_number_async().fuse(); // mut becauee multypel times polling.
        let num2 = num2().fuse();
        let num3 = num3().fuse();

        futures::pin_mut!(num1, num2, num3);

        futures::select! {
            x = num1 => println!("num1 completed {}", x),
            x = num2 => println!("num2 completed {}", x),
            x = num3 => println!("num3 completed {}", x),
            complete => {
                println!("All functions completed.");
            }
        }
    });

    // println!("{:?}", res);
}

async fn get_number_async() -> u8 {
    println!("Running");
    8
}

async fn num2() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(800));
    23
}

async fn num3() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(40));
    39
}

// Y: another way to do
// fn another_way_to_async() -> impl Future<u8> {}
