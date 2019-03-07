
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

    fn move_toward_player() {
        //every t=10 seconds, move one square towards the current shortest path towards the player.
    }
}