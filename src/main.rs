use std::{collections::HashMap};

use macroquad::{prelude::*};
use std::sync::Mutex;
use once_cell::sync::Lazy;

static WIDTH: f32 = 960.0;
static HEIGHT: f32 = 960.0;

static HALF_WIDTH: f32 = WIDTH/2.0;
static HALF_HEIGHT: f32 = HEIGHT/2.0;

static ACCELERATION: f32 = 5.0;
static DRAG_MULT: f32 = 0.01;

static MAXSPEED: f32 = 200.0;

static INPUT_DELAY: f64 = 0.1;


static DIR2ANGLE: Lazy<Mutex<HashMap<u16, i16>>> = Lazy::new(|| {
    let mut m = HashMap::new();
	m.insert(DIRS.north, 90);
	m.insert(DIRS.south, -90);
	m.insert(DIRS.west, 180);
	m.insert(DIRS.east, 0);
	m.insert(DIRS.northwest, 135);
	m.insert(DIRS.northeast, 45);
	m.insert(DIRS.southwest, -135);
	m.insert(DIRS.southeast, -45);
    Mutex::new(m)
});


struct Delay {
	x: f64,
	y: f64
}
struct Dir {
	north: u16,
	south: u16,
	west: u16,
	east: u16,
	northwest: u16,
	northeast: u16,
	southwest: u16,
	southeast: u16
}

static DIRS: Dir = Dir{
	north: 0x1000,
	south: 0x0100,
	west: 0x0010,
	east: 0x0001,
	northwest: 0x1010,
	northeast: 0x1001,
	southwest: 0x0110,
	southeast: 0x0101
};

trait Inertia 
{
	fn calc_vector_inertia(&mut self, delay: &Delay, weight: f32) {}		
}

impl Inertia for Vec2 {
	fn calc_vector_inertia(&mut self, delay: &Delay, weight: f32) {
		let time = get_time();
		if delay.x <= time {
			if self.x.abs() <= 5.0 {
				self.x = 0.0;
			} else {
				self.x = interpolate(self.x, 0.0, weight);
			}
		}
		if delay.y <= time {
			if self.y.abs() <= 5.0 {
				self.y = 0.0;
			} else {
				self.y = interpolate(self.y , 0.0, weight);
			}
		}
		
	}
}

fn interpolate(a: f32, b: f32, w: f32) -> f32 {
	a + (b - a) * w
}

fn force(result_vector: &mut Vec2, delay: &mut Delay, tick: &f64, norm_force_vector: &mut Vec2) {
	if norm_force_vector.x.abs() > 0.0 {delay.x = tick + INPUT_DELAY;}
	if norm_force_vector.y.abs() > 0.0 {delay.y = tick + INPUT_DELAY;}
	*norm_force_vector *= ACCELERATION;
	*norm_force_vector += *result_vector;
	let hip = norm_force_vector.length();
	result_vector.x = norm_force_vector.x / hip * hip.clamp(-MAXSPEED, MAXSPEED);
	result_vector.y = norm_force_vector.y / hip * hip.clamp(-MAXSPEED, MAXSPEED);
}

fn process_force_adj(force_vector: &mut Vec2, result_vector: &mut Vec2, delay: &mut Delay, tick: &f64) {
	let mut keybit: u16 = 0;	
	let mut angle: Option<i16> = Option::None;
	keybit += DIRS.north * is_key_down(KeyCode::Up) as u16;
	keybit += DIRS.south * is_key_down(KeyCode::Down) as u16;
	keybit += DIRS.east * is_key_down(KeyCode::Right) as u16;
	keybit += DIRS.west * is_key_down(KeyCode::Left) as u16;
	match DIR2ANGLE.lock().unwrap().get(&keybit) {
		Some(r) => {angle.insert(*r);},	
		None => {}
	}
	if angle.is_some() {
		let mut angle_un: f32 = angle.unwrap() as f32;
		angle_un *= std::f32::consts::PI / 180.0;
		let mut norm_force_vector: Vec2 = Vec2::new(0.0, 0.0);
		norm_force_vector.x = angle_un.cos();
		norm_force_vector.y = angle_un.sin();
		force_vector.x = norm_force_vector.x * MAXSPEED;
		force_vector.y = norm_force_vector.y * MAXSPEED;
		force(result_vector, delay, tick, &mut norm_force_vector);
	} else if is_mouse_button_down(MouseButton::Left) {
		let (pos_x, pos_y) = mouse_position();
		force_vector.x = pos_x - HALF_WIDTH;
		force_vector.y = HALF_HEIGHT - pos_y;
		let mut norm_force_vector: Vec2 = force_vector.normalize();
		force(result_vector, delay, tick, &mut norm_force_vector);
	} else {
		force_vector.x = 0.0;
		force_vector.y = 0.0;
	}
}

async fn draw_plane() {
	draw_line(HALF_WIDTH, 0.0, HALF_WIDTH, HEIGHT, 2.0, BLACK);
	draw_line(0.0, HALF_HEIGHT, WIDTH, HALF_HEIGHT, 2.0, BLACK);
	let mut chert_low = HALF_WIDTH - 5.0;
	let mut chert_max = HALF_WIDTH + 5.0;
	let mut cord_text: String;
	let mut x: f32;
	let mut y: f32;
	for _x in (0..WIDTH as i16).step_by(30) {
		x = _x as f32;
		draw_line(x as f32, chert_low, x as f32, chert_max, 1.0, BLACK);
		cord_text = format!("{}", x - HALF_WIDTH);
		draw_text(&cord_text, x as f32, HALF_HEIGHT+20.0, 14.0, BLACK);
	}
	chert_low = HALF_HEIGHT - 5.0;
	chert_max = HALF_HEIGHT + 5.0;
	for _y in (0..HEIGHT as i16).step_by(30) {
		y = _y as f32;
		draw_line(chert_low, y as f32, chert_max, y as f32, 1.0, BLACK);
		cord_text = format!("{}",  HALF_HEIGHT - y);
		draw_text(&cord_text, HALF_WIDTH-30.0, y as f32, 14.0, BLACK);
	}
}

async fn draw_vectors(force_vector: &Vec2, main_vector: &Vec2) {
	if main_vector.length() > 0.0 {
		draw_line(HALF_WIDTH, HALF_HEIGHT, HALF_WIDTH + main_vector.x, HALF_HEIGHT - main_vector.y, 5.0, BLACK);
	}
	if force_vector.length() > 0.0 {
		draw_line(HALF_WIDTH, HALF_HEIGHT, HALF_WIDTH + force_vector.x, HALF_HEIGHT - force_vector.y, 3.0, RED);
	}
}

async fn draw_info(force_vector: &Vec2, result_vector: &Vec2) {
	let force_vector_info: String = format!("[FORCE] X:{} Y:{} Hip:{}", 
											force_vector.x.round(), force_vector.y.round(), force_vector.length().round());
	let result_vector_info: String = format!("[RESULT] X:{}, Y:{}, Hip: {}",
											result_vector.x.round(), result_vector.y.round(), result_vector.length().round());
	draw_text(&force_vector_info, 40.0, 40.0, 20.0, BLACK);
	draw_text(&result_vector_info, 40.0, 80.0, 20.0, BLACK);

}

#[macroquad::main("Vectorize")]
async fn main() {
	request_new_screen_size(WIDTH, HEIGHT);
	let mut force_vector: Vec2 = Vec2::new(0.0, 0.0);
	let mut result_vector: Vec2 = Vec2::new(0.0, 0.0);
	let mut delay: Delay = Delay {x: 0.0, y: 0.0};
    let mut tick: f64;
	loop {
		tick = get_time(); 
        clear_background(WHITE);
		draw_plane().await;
		process_force_adj(&mut force_vector, &mut result_vector, &mut delay, &tick);
		result_vector.calc_vector_inertia(&delay, DRAG_MULT);
		draw_info(&force_vector, &result_vector).await;
		draw_circle_lines(HALF_WIDTH, HALF_HEIGHT, MAXSPEED, 1.0, RED);
		draw_vectors(&force_vector, &result_vector).await;
        next_frame().await
    }
}