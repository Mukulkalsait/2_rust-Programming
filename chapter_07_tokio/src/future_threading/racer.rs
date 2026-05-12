pub struct Racer {
    pub racer_name: String,
    pub completed_laps: u8,
    pub total_laps: u8,
    pub best_lap_time: u16,
    pub lap_timings: Vec<u16>,
}

impl Racer {
    pub fn new(name: String, c_laps: u8, t_laps: u8, laps_timings: Vec<u16>) -> Racer {
        Racer { racer_name: name, completed_laps: c_laps, total_laps: t_laps, best_lap_time: 1000, lap_timings: laps_timings }
    }

    pub fn do_lap(&mut self) {
        let lap_time = self.lap_timings.pop();
        if lap_time.is_some() && lap_time.unwrap() < self.best_lap_time {
            self.best_lap_time = lap_time.unwrap();
        }
        self.completed_laps += 1;
    }
}

impl std::future::Future for Racer {
    type Output = u16; // Y: we only need the "best_lap_time" HENCE onlyl u8

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        print!("|{:?}|", std::thread::current().id());
        if self.completed_laps < self.total_laps {
            print!("|{}|Lap Started|>", self.racer_name);
            self.get_mut().do_lap();
            println!("[Completed]");
            cx.waker().wake_by_ref(); // wakeup the function to run again.
            return std::task::Poll::Pending;
        }

        println!("Best Lap time for {} is :{}", self.racer_name, self.best_lap_time);
        std::task::Poll::Ready(self.best_lap_time)
    }
}
