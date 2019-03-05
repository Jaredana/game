/* TODO:

make collision and player pos resseting actually work
make grid adjust to screen size such that the number of squares is based on screen size
add enemies

*/
extern crate piston_window;
extern crate find_folder;
use piston_window::*;
use std::string::ToString;
mod player;
mod grid;
use player::Player;
use grid::Grid;

//this should be a member function
fn player_pos_to_string(pos: [f64; 4])  -> String{
	let mut x = pos[0].to_string();
	let y = pos[1].to_string();
	x.push_str(",");
	x.push_str(&y);
	return x;
}

fn main() {
    let mut player = Player::new();

    //need to set screen_size to monitor resolution
	let screen_size = [1920,1080];
    let mut window: PistonWindow = WindowSettings::new("Hello, Piston!", screen_size)
    	.exit_on_esc(true)
    	.vsync(true)
		.fullscreen(false)
    	.build().unwrap();
    //get assets for text
    let assets = find_folder::Search::ParentsThenKids(3,3)
    	.for_folder("assets").unwrap();
    println!("{:?}", assets);
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
    //Clear scren and draw rectangle
    while let Some(event) = window.next() {
    	window.draw_2d(&event, |context, graphics| {
    		clear([1.0; 4], graphics);
    		let transform = context.transform.trans(10.0, 100.0);
            
            //check if player is in a valid location before rendering, if not, fix location.
            if player.pos_is_valid(screen_size) {
                rectangle(player.color, player.position, context.transform, graphics);
            }
    		else {
                player.reset_pos(screen_size)
            }
    		let my_text = text::Text::new_color([0.0,0.0,0.0,1.0], 32).draw(&player_pos_to_string(player.position), &mut glyphs, &context.draw_state, transform, graphics);
				match my_text {
					Result::Ok(val) => val,
					Result::Err(err) =>
						panic!("text didnt render, err {:?}", err)
				}
			//this is where grid can be fit to screen size(set to 1080p manually now) cols = steps left or right 1920/120; rows = steps up or down
    		let grid = Grid {
    			cols: (screen_size[0]  / player.position[3] as u32) ,
    			rows: (screen_size[1]  / player.position[3] as u32),
    			units: player.position[3],
    		};
    		//we need to draw a line for every 20 units of x and 20 units of y;
    		let line = Line::new([1.0,0.0,0.0,1.0], 1.0);
    		grid.draw(&line, &context.draw_state, context.transform, graphics)
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
                player.position[0] += updated_pos.0;
                player.position[1] += updated_pos.1;
		};
	}
}
