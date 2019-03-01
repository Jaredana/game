/* TODO:

make collision and player pos resseting actually work
make grid adjust to screen size such that the number of squares is based on screen size
add enemies

*/
extern crate piston_window;
extern crate find_folder;
use piston_window::*;
use std::string::ToString;
use math::{Matrix2d, Scalar, Vec2d};
use {DrawState, Graphics, Line};
mod player;
use player::Player;



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
    let screen_size = [1920,1080];
    //Create Game Window
    let mut window: PistonWindow = WindowSettings::new("Hello, Piston!", screen_size)
    	.exit_on_esc(true)
    	.vsync(true)
    	.build().unwrap();
    
    //get assets for text
    let assets = find_folder::Search::ParentsThenKids(3,3)
    	.for_folder("assets").unwrap();
    println!("{:?}", assets);
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();
    println!("{:?}", window.size().height);
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
    		text::Text::new_color([0.0,0.0,0.0,1.0], 32).draw(&player_pos_to_string(player.position), &mut glyphs, &context.draw_state, transform, graphics);
    		
    		let grid = Grid {
    			cols: 16,
    			rows: 9,
    			units: 120.0,
    		};
    		//we need to draw a line for every 20 units of x and 20 units of y; eventually we should find formula to base it on window size
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
            //move player
			player.position[0] += updated_pos.0;
			player.position[1] += updated_pos.1;
		};
	}
}
#[derive(Debug, Copy, Clone)]
pub struct Grid {
    /// Number of columns.
    pub cols: u32,
    /// Number of rows.
    pub rows: u32,
    /// The width and height of each grid cell.
    pub units: Scalar,
}

/// Iterates through the cells of a grid as (u32, u32).
#[derive(Debug, Copy, Clone)]
pub struct GridCells {
    cols: u32,
    rows: u32,
    state: u64,
}

impl Grid {
    /// Draws the grid.
    pub fn draw<G>(&self, line: &Line, draw_state: &DrawState, transform: Matrix2d, g: &mut G)
        where G: Graphics
    {
        let &Grid { cols, rows, units } = self;
        for x in 0..cols + 1 {
            let x1 = x as Scalar * units;
            let y1 = 0.0;
            let x2 = x1;
            let y2 = rows as Scalar * units;
            line.draw([x1, y1, x2, y2], draw_state, transform, g);
        }
        for y in 0..rows + 1 {
            let x1 = 0.0;
            let y1 = y as Scalar * units;
            let x2 = cols as Scalar * units;
            let y2 = y1;
            line.draw([x1, y1, x2, y2], draw_state, transform, g);
        }
    }

    /// Get a GridIterator for the grid
    pub fn cells(&self) -> GridCells {
        GridCells {
            cols: self.cols,
            rows: self.rows,
            state: 0,
        }
    }

    /// Get on-screen position of a grid cell
    pub fn cell_position(&self, cell: (u32, u32)) -> Vec2d {
        [cell.0 as Scalar * &self.units, cell.1 as Scalar * &self.units]
    }

    /// Get on-screen x position of a grid cell
    pub fn x_pos(&self, cell: (u32, u32)) -> Scalar {
        self.cell_position(cell)[0]
    }

    /// Get on-screen y position of a grid cell
    pub fn y_pos(&self, cell: (u32, u32)) -> Scalar {
        self.cell_position(cell)[1]
    }
}

impl Iterator for GridCells {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        let cols = self.cols as u64;
        let rows = self.rows as u64;

        if self.state == cols * rows {
            return None;
        }

        // reverse of: state = x + (y * cols)
        let ret = ((self.state % cols) as u32, (self.state / cols) as u32);
        self.state += 1;

        return Some(ret);
    }
}