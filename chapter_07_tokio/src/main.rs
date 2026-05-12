mod future_threading;
use future_threading::racer::Racer;

/// test fucntion
async fn _test_abcd() {
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("working.");
}

/// Racer multythreeading, sybertenus threading, polling, future usage, async all in one.
fn race_start() {
    let racer_mukul = Racer::new("mukul".to_string(), 0, 8, vec![433, 123, 224, 958, 893, 549, 293, 593]);
    let racer_kaiwaly = Racer::new("kaiwaly".to_string(), 0, 8, vec![433, 329, 224, 958, 893, 549, 293, 593]);

    let handle_mukul = tokio::task::spawn(racer_mukul);
    let handle_kaiwaly = tokio::task::spawn(racer_kaiwaly);

    loop {
        if handle_mukul.is_finished() && handle_kaiwaly.is_finished() {
            println!("All Racers Are Finished ");
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
}

// -------------------------------------------------------------------------------------------------------------------
// Y:
// #[tokio::main(flavor = "multi_thread", worker_threads = 2)] // total threads assign
// #[tokio::main(flavor = "current_thread")] // single threaded
// default Multi Threaded but  with flavor = "current_thread" its Single Threaded
#[tokio::main(flavor = "multi_thread", worker_threads = 12)]
async fn main() {
    // test_abcd().await;
    race_start();
}
