/* TODO:

make collision and player pos resseting actually work
make grid adjust to screen size such that the number of squares is based on screen size
add enemies
make borders solid so player cannot go through them

*/

//CHECK IF PLAYER LOCATION IS VALID IF NOT SEND THEM TO THE CLOSEST VALID LOCATION	
//aka checking player.position to see if x,y are in bounds of screen
extern crate piston_window;
extern crate piston;
extern crate find_folder;
extern crate winit;
extern crate timer_controller;
//use std::sync::mpsc::{Sender, Receiver};
//use std::sync::mpsc;
use std::sync::mpsc::channel;
//use std::io;
use winit::EventsLoop;
use piston_window::*;
use std::string::ToString;

mod player;
mod grid;
mod enemy;
use enemy::Enemy;
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
	let mut enemy = Enemy::new();
	
	//works but is so janked out
	let (sender, receiver)= channel();
	enemy.enemy_timer(sender.clone());
	let init_msg = receiver.recv().unwrap();
	

	//get physical size of monitor
	let test = EventsLoop::new();
	let test2 = test.get_primary_monitor();
    let screen_size = [test2.get_dimensions().height as u32,test2.get_dimensions().width as u32];
    //Create Game Window
    let mut window: PistonWindow = WindowSettings::new("Hello, Piston!", screen_size)
    	.exit_on_esc(true)
    	.vsync(true)
		.fullscreen(true)
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
				rectangle(enemy.color, enemy.position, context.transform, graphics);
            }
    		else {
                player.reset_pos(screen_size)
            }
			let msg = receiver.recv().unwrap();
			if(msg != init_msg){
				println!("{:?}", msg);
			}
    		let my_text = text::Text::new_color([0.0,0.0,0.0,1.0], 32).draw(&player_pos_to_string(player.position), &mut glyphs, &context.draw_state, transform, graphics);
				match my_text {
					Result::Ok(val) => val,
					Result::Err(err) =>
						panic!("text didnt render, err {:?}", err)
				}
    		let grid = Grid {
    			cols: 16,
    			rows: 9,
    			units: 120.0,
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
