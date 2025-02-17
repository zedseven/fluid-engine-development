//! 1.1. Hello, Fluid Simulator

use std::{
	f64::consts::PI,
	io::{stdout, Write},
	thread::sleep,
	time::Duration,
};

// Display Constants
const HEIGHT_FIELD_SIZE: usize = 80;
const GRAYSCALE_TABLE: &[char] = &[' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

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
	let start = ((position - quarter_wave_length) * field_size as f64) as isize;
	let end = ((position + quarter_wave_length) * field_size as f64) as isize;

	for field_index in start..end {
		// Boundary Reflection
		let reflected_field_index = if field_index < 0 {
			(-field_index - 1) as usize
		} else if field_index >= field_size {
			(2 * field_size - field_index - 1) as usize
		} else {
			field_index as usize
		};

		let distance = ((field_index as f64 + 0.5) / field_size as f64 - position).abs();
		let height = max_height * 0.5 * ((distance * PI / quarter_wave_length).min(PI).cos() + 1.0);
		height_field[reflected_field_index] += height;
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
	for _ in 0..field_size {
		print!("\x08");
	}

	// Draw the new buffer
	print!("{}", buffer);
	stdout().flush().expect("unable to flush stdout");
}

// Entry Point
fn main() {
	const SIMULATION_TIME_PERIOD_SECONDS: u32 = 1000;
	const FRAMES_PER_SECOND: u32 = 120;
	const FRAME_TIME_INTERVAL: f64 = 1.0 / FRAMES_PER_SECOND as f64;

	let wave_length_x = 0.8;
	let max_height_x = 0.5;
	let wave_length_y = 1.2;
	let max_height_y = 0.4;

	let mut position_x = BOUNDARY_LEFT;
	let mut speed_x = 1.0;
	let mut position_y = BOUNDARY_RIGHT;
	let mut speed_y = -0.5;

	let mut height_field = vec![0.0; HEIGHT_FIELD_SIZE];

	// Main Loop
	let sleep_duration = Duration::from_secs_f64(FRAME_TIME_INTERVAL);
	for _ in 0..(SIMULATION_TIME_PERIOD_SECONDS * FRAMES_PER_SECOND) {
		// March through time
		update_wave(FRAME_TIME_INTERVAL, &mut position_x, &mut speed_x);
		update_wave(FRAME_TIME_INTERVAL, &mut position_y, &mut speed_y);

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
