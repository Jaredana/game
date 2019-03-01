pub struct Player {
	pub color: [f32; 4],
	pub position: [f64; 4],
	pub speed: f64,
}

impl Player {
	pub fn new() -> Self {
		Player {
			color: [1.0,0.0, 0.0, 1.0],
			position: [0.0, 0.0, 120.0, 120.0],
			speed: 12.0 //adjust speed here, should be calc from GCF of screen size/10
		}
	}
	//needs to know screen size and adjust accordingly
	pub fn pos_is_valid(&mut self, screen: [u32; 2]) -> bool{
		if (self.check_x(screen) == 0.0) & (self.check_y(screen) == 0.0)  {
			return true;
		}
		return false;
	}
    fn check_x(&mut self, screen: [u32;2]) -> f64{
        if self.position[0] >= 0.0 {
            return 0.0;
        }
        else if self.position[0] <= (screen[0] as f64 - self.position[3]) as f64 {

            return 0.0;
        } 
        else{
            return self.get_diff(self.position[0], screen[0]) 
        }
    }
    fn check_y(&mut self, screen: [u32;2]) -> f64{
        if self.position[1] >= 0.0 {
            return 0.0;
        } 
        else if self.position[1] <= (screen[1] as f64 - self.position[3]) as f64{
            return 0.0;
        } else {
            return self.get_diff(self.position[1], screen[1]);
        }
    }
    fn get_diff(&self, pos: f64, screen: u32) -> f64{
        return screen as f64 + pos
    }
	//this function needs to be able to tell which position is the one out of range and reset it to nearest min/max for respective x,y.
	pub fn reset_pos(&mut self, screen: [u32;2]){
        let x = self.check_x(screen);
        let y = self.check_y(screen);
        //X is good Y is not
        if (x == 0.0) & (y != 0.0) {
            //we need to find if player.position[0] is closer to 0 or screen[0]-player.position[3]
            self.position[1] = y;
        }
        //X is not Y is good
        else if (self.check_x(screen) != 0.0) & (self.check_y(screen) == 0.0){
            self.position[0] = x;
        }
        //Neither are in-bounds
        else {
            //need a way to compare diff on both numbers
        }
	}
}