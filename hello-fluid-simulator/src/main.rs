//! 1.1. Hello, Fluid Simulator

use std::{
	f64::consts::PI,
	io::{stdout, Write},
	thread::sleep,
	time::Duration,
};

// Display Constants
const HEIGHT_FIELD_SIZE: usize = 80;
const GRAYSCALE_TABLE: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

// Simulation Constants
const BOUNDARY_LEFT: f64 = 0.0;
const BOUNDARY_RIGHT: f64 = 1.0;

fn update_wave(time_interval: f64, position: &mut f64, speed: &mut f64) {
	*position += time_interval * *speed;

	// Boundary Reflection
	if *position > BOUNDARY_RIGHT {
		*speed *= -1.0;
		*position = BOUNDARY_RIGHT + time_interval * *speed;
	} else if *position < BOUNDARY_LEFT {
		*speed *= -1.0;
		*position = BOUNDARY_LEFT + time_interval * *speed;
	}
}

fn accumulate_wave_to_field(
	position: f64,
	wave_length: f64,
	max_height: f64,
	height_field: &mut Vec<f64>,
) {
	let field_size = height_field.len() as isize;
	let quarter_wave_length = 0.25 * wave_length;
	let start = (position - quarter_wave_length) as isize * field_size;
	let end = (position + quarter_wave_length) as isize * field_size;

	for i in start..end {
		let j = if i < 0 {
			(-i - 1) as usize
		} else if i >= field_size {
			(2 * field_size - i - 1) as usize
		} else {
			i as usize
		};

		let distance = (i as f64 + 0.5).abs() / field_size as f64 - position;
		let height = max_height * 0.5 * ((distance * PI / quarter_wave_length).min(PI).cos() + 1.0);
		height_field[j] += height;
	}
}

fn draw(height_field: &Vec<f64>) {
	let table_size = GRAYSCALE_TABLE.len();
	let field_size = height_field.len();
	let mut buffer = String::new();

	// Convert height field to grayscale
	for i in 0..field_size {
		let height = height_field[i];
		let table_index = ((table_size as f64 * height) as usize).min(table_size - 1);
		buffer.push(GRAYSCALE_TABLE[table_index]);
	}

	// Clear old prints
	/*for _ in 0..field_size {
		print!("\x08");
	}*/
	println!();

	// Draw the new buffer
	print!("{}", buffer);
	stdout().flush().expect("unable to flush stdout");
}

// Entry Point
fn main() {
	const FPS: u32 = 100;
	const TIME_INTERVAL: f64 = 1.0 / FPS as f64;

	let wave_length_x = 0.8;
	let wave_length_y = 1.2;
	let max_height_x = 0.5;
	let max_height_y = 0.4;

	let mut position_x = 0.0;
	let mut position_y = 0.0;
	let mut speed_x = 1.0;
	let mut speed_y = -0.5;

	let mut height_field = vec![0.0; HEIGHT_FIELD_SIZE];

	// Main Loop
	let sleep_duration = Duration::from_secs_f64(TIME_INTERVAL);
	for _ in 0..1000 {
		// March through time
		update_wave(TIME_INTERVAL, &mut position_x, &mut speed_x);
		update_wave(TIME_INTERVAL, &mut position_y, &mut speed_y);

		// Clear height field
		for height in &mut height_field {
			*height = 0.0;
		}

		// Accumulate waves for each center point
		accumulate_wave_to_field(position_x, wave_length_x, max_height_x, &mut height_field);
		accumulate_wave_to_field(position_y, wave_length_y, max_height_y, &mut height_field);

		// Draw the height field
		draw(&height_field);

		// Wait
		sleep(sleep_duration);
	}

	println!();
}
