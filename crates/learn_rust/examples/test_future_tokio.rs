use std::thread::sleep;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let racer1 = F1Racer::new("Hamilton".to_string());
    let mut racer2 = F1Racer::new("Verstappen".to_string());

    racer2.lap_times.pop();
    racer2.lap_times.push(10);

    let handle1 = tokio::spawn(racer1);
    let handle2 = tokio::spawn(racer2);

    let _ = handle1.await;
    let _ = handle2.await;
}

struct F1Racer {
    lap_times: Vec<u8>,
    best_lap_time: u8,
    name: String,
    laps: u8,
    lap: u8,
}

impl F1Racer {
    fn new(name: String) -> Self {
        F1Racer {
            lap_times: vec![222, 70, 99, 20, 55],
            best_lap_time: 255,
            name,
            laps: 5,
            lap: 0,
        }
    }

    fn do_lap(&mut self) {
        self.lap += 1;
        sleep(std::time::Duration::from_secs(1));
        let lap_time = self.lap_times.pop();
        println!("{} lap time: {:?}", self.name, lap_time);
        if lap_time.is_some() && self.best_lap_time > lap_time.unwrap() {
            self.best_lap_time = lap_time.unwrap();
        }
    }
}

impl std::future::Future for F1Racer {
    type Output = u8;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        println!(
            "{}, Thread id: {:?}",
            self.name,
            std::thread::current().id()
        );
        if self.lap < self.laps {
            self.get_mut().do_lap();
            cx.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        println!("{} best lap time: {:?}", self.name, self.best_lap_time);
        return std::task::Poll::Ready(self.best_lap_time);
    }
}
