/* TODO:

make collision and player pos resseting actually work
print player pos to screen
add enemies

*/
extern crate piston_window;
use piston_window::*;

struct Player {
	pub color: [f32; 4],
	pub position: [f64; 4],
	pub speed: f64,
}

impl Player {
	fn new() -> Self {
		Player {
			color: [1.0,0.0, 0.0, 1.0],
			position: [0.0, 0.0, 100.0, 100.0],
			speed: 5.0 //adjust speed here
		}
	}

	fn pos_is_valid(&mut self) -> bool{
		if (self.position[0] >= 0.0) & (self.position[0] <= 640.0 )& (self.position[1] >= 0.0) & (self.position[1] <= 480.0)  {
			return true;
		}
		return false;
	}
	fn reset_pos(&mut self){
		self.position[0] = 320.0;
		self.position[1] = 240.0;
	}
}

fn main() {
    let mut player = Player::new();
    
    //Create Game Window
    let mut window: PistonWindow = WindowSettings::new("Hello, Piston!", [640,480])
    	.exit_on_esc(true)
    	.vsync(true)
    	.build().unwrap();
    
    //Clear scren and draw rectangle
    while let Some(event) = window.next() {
    	window.draw_2d(&event, |context, graphics| {
    		clear([1.0; 4], graphics);
    		rectangle(player.color, player.position, context.transform, graphics);
    	});
  
		if let Some(Button::Keyboard(k)) = event.press_args() {
			let updated_pos = match k {
				Key::W => {
					(0.0, -10.0*player.speed) //Up
				}
				Key::S => {
					(0.0, 10.0*player.speed)//Down
				}
				Key::A => {
					(-10.0*player.speed, 0.0) //Left
				}
				Key::D => {
					(10.0*player.speed,0.0) //Right
				}
				_ => (0.0, 0.0)
		};

		if player.pos_is_valid() {
			player.position[0] += updated_pos.0;
			player.position[1] += updated_pos.1;
		}
		else{
			player.reset_pos()
		}

		};
	}
}