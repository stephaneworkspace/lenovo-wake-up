use std::{
    thread,
    time::{Duration, Instant},
};
use wake_on_lan;

/// Thread timer for loop evry 60sec
fn main() {
    let scheduler = thread::spawn(|| {
        let wait_time = Duration::from_millis(500);

        // Make this an infinite loop
        // Or some control path to exit the loop
        for _ in 0..100 {
            let start = Instant::now();
            eprintln!("Scheduler starting at {:?}", start);

            let thread_wake = thread::spawn(wake);
            //let thread_b = thread::spawn(b);

            thread_wake.join().expect("Thread A panicked");
            //thread_b.join().expect("Thread B panicked");

            let runtime = start.elapsed();

            if let Some(remaining) = wait_time.checked_sub(runtime) {
                eprintln!(
                    "schedule slice has time left over; sleeping for {:?}",
                    remaining
                );
                thread::sleep(remaining);
            }
        }
    });

    scheduler.join().expect("Scheduler panicked");
}

fn wake() {
    println!("Wakeup my server");
    let mac_adress: [u8; 6] = [0x44, 0x8A, 0x5B, 0x7F, 0x9B, 0x7F];
    let magic_packet = wake_on_lan::MagicPacket::new(&mac_adress);
    match magic_packet.send() {
        Err(e) => eprintln!("{:?}", e),
        _ => eprintln!("packet send"),
    }
    // thread::sleep(Duration::from_millis(100))
    thread::sleep(Duration::from_secs(60))
}
