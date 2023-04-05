use macroquad::{prelude::*};
include!("utils.rs");
include!("fn.rs");

async fn draw_plane() {
	draw_line(WIDTH/2.0, 0.0, WIDTH/2.0, HEIGHT, 2.0, BLACK);
	draw_line(0.0, HEIGHT/2.0, WIDTH, HEIGHT/2.0, 2.0, BLACK);


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

async fn draw_vectors(main_vector: &Vec2, force_vector: &Vec2) {
	if main_vector.length() > 0.0 {
		draw_line(HALF_WIDTH, HALF_HEIGHT, HALF_WIDTH + main_vector.x, HALF_HEIGHT - main_vector.y, 5.0, BLACK);
	}
	if force_vector.length() > 0.0 {
		draw_line(HALF_WIDTH, HALF_HEIGHT, HALF_WIDTH + force_vector.x, HALF_HEIGHT - force_vector.y, 5.0, BLACK);
	}
}

#[macroquad::main("Vectorize")]
async fn main() {
	request_new_screen_size(WIDTH, HEIGHT);
	let mut force_vector: Vec2 = Vec2::new(0.0, 0.0);
	let mut result_vector: Vec2 = Vec2::new(200.0, 200.0);
	let mut delay: Delay = Delay {x: 0.0, y: 0.0};
    let mut tick: f64;
	loop {
		tick = get_time();
        clear_background(WHITE);
		draw_plane().await;
		result_vector.calc_vector_inertia(&delay, DRAG_MULT);
		if is_mouse_button_down(MouseButton::Left) {
			let (pos_x, pos_y) = mouse_position();
			force_vector.x = pos_x - HALF_WIDTH;
			force_vector.y = HALF_HEIGHT - pos_y;
			let mut norm_force_vector: Vec2 = force_vector.normalize();
			if norm_force_vector.x.abs() > 0.0 {
				delay.x = tick + INPUT_DELAY;
			}
			if norm_force_vector.y.abs() > 0.0 {
				delay.y = tick + INPUT_DELAY;
			}

			norm_force_vector *= Vec2::new(ACCELERATION, ACCELERATION);
			result_vector += norm_force_vector;
		} else {
			force_vector.x = 0.0;
			force_vector.y = 0.0;
		}
		draw_vectors(&result_vector, &force_vector).await;
        next_frame().await
    }
}