#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
// #[tokio::main(flavor = "current_thread")]
// default Multi Threaded but  with flavor = "current_thread" its Single Threaded
async fn main() {
    // test_abcd().await;
}

async fn _test_abcd() {
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("working.");
}

struct Racer {
    name: String,
    completed_laps: u8,
    total_laps: u8,
    best_lap_time: u8,
    lap_times: Vec<u8>,
}

impl Racer {
    fn new() -> Racer {
        Racer { name: "Mukul".to_string(), completed_laps: 0, total_laps: 5, best_lap_time: 255, lap_times: vec![90, 123, 54, 245, 145] }
    }

    fn do_lap(&mut self) {
        let lap_time = self.lap_times.pop();
        if lap_time.is_some() && lap_time.unwrap() < self.best_lap_time {
            self.best_lap_time = lap_time.unwrap();
        }
        self.completed_laps += 1;
    }
}

impl std::future::Future for Racer {
    type Output = u8; // Y: we only need the "best_lap_time" HENCE onlyl u8

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.completed_laps < self.total_laps {
            self.get_mut().do_lap();
            return std::task::Poll::Pending;
        }

        std::task::Poll::Ready(self.best_lap_time)
    }
}
