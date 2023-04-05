
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

static WIDTH: f32 = 960.0;
static HEIGHT: f32 = 960.0;

static HALF_WIDTH: f32 = WIDTH/2.0;
static HALF_HEIGHT: f32 = HEIGHT/2.0;

static ACCELERATION: f32 = 5.0;
static DRAG_MULT: f32 = 0.01;

static INPUT_DELAY: f64 = 0.1;