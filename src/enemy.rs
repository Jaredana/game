use std::time::{Duration, SystemTime};
use std::sync::mpsc::{Sender, Receiver};
//use std::sync::mpsc::channel;
//use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
pub struct Enemy {
	pub color: [f32; 4],
	pub position: [f64; 4],
	pub speed: f64,
}
impl Enemy {
    pub fn new() -> Self {
		Enemy {
			color: [0.3,0.5, 2.0, 1.0], //blue
			position: [120.0, 240.0, 120.0, 120.0], //(X, Y, Length, Width)
			speed: 12.0 //adjust speed here, should be calc from GCF of screen size/10
		}
	}

    pub fn enemy_timer(&self, sx: Sender<u32>) {
        thread::spawn(move || {
            let now = SystemTime::now();
            while 1==1 {
                // we sleep for 10 seconds
                sleep(Duration::new(10, 0));
                match now.elapsed() {
                    Ok(elapsed) => {
                        // it prints '10'
                        //println!("{}", elapsed.as_secs());
                        sx.send(elapsed.as_secs() as u32).unwrap();
                    }
                    Err(e) => {
                        // an error occurred!
                        println!("Error: {:?}", e);
                    }
                }
            }
        });
        //child.join().expect("Oops, error");
    }
}