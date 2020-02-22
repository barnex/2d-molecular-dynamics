extern crate rand;
extern crate sdl2;

use atoms::prelude::*;
use atoms::sim::World;
use rand::random;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::EventPump;
use std::error;
use std::path::Path;

pub fn main() -> Result<()> {
	// ---- set up world
	let width = 120.0;
	let height = 80.0;
	let mut w = World::new(width, height);

	// ---- initial state
	let lattice = 0.50;
	let v = 9.9;
	for i in 0..64 {
		for j in 0..32 {
			let x = linterp(0.0, 0.0, 32.0, height * lattice, i as f32);
			let y = linterp(0.0, 0.0, 32.0, height * lattice, j as f32);
			let vx = (random::<f32>() - 0.5) * v;
			let vy = (random::<f32>() - 0.5) * v;
			w.push(Atom::with_v(x, y, vx, vy));
		}
	}

	let (mut canvas, mut event_pump) = init_window()?;

	// ---- set up assets
	let surface = sdl2::surface::Surface::load_bmp(Path::new("assets/atom2-8.bmp"))?;
	let texture_creator = canvas.texture_creator();
	let texture = texture_creator.create_texture_from_surface(&surface)?;

	// show initial state
	render_to(&w, &texture, &mut canvas)?;

	// wait for user to press start
	println!("Press any key to start...");
	await_any_input(&mut event_pump);

	// animate
	'running: loop {
		// ---- event loop
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				} => break 'running,
				_ => {}
			}
		}

		// ----- update and render
		//let start = time::Instant::now();
		w.update_steps(50, 0.002);
		//let update_t = start.elapsed();

		//let start = time::Instant::now();
		render_to(&w, &texture, &mut canvas)?;
		//let render_t = start.elapsed();

		//println!(
		//	"calc: {}s, render: {}s",
		//	update_t.as_secs_f32(),
		//	render_t.as_secs_f32()
		//);
	}
	Ok(())
}

// blocks until any key or mouse button is pressed.
// used to start the animation.
fn await_any_input(event_pump: &mut EventPump) {
	loop {
		match event_pump.wait_event() {
			Event::KeyDown { .. } => break,
			Event::MouseButtonDown { .. } => break,
			_ => (),
		}
	}
}

fn render_to(w: &World, texture: &Texture, canvas: &mut Canvas) -> Result<()> {
	let bg = Color::RGB(255, 255, 255);
	canvas.set_draw_color(bg);
	canvas.clear();

	let canvas_size = canvas.output_size()?;
	let pix_x = canvas_size.0;
	let pix_y = canvas_size.1;

	let pix = |x, y| {
		(
			linterp(0.0, 0.0, w.w(), pix_x as f32, x) as i32 - 8,
			linterp(0.0, pix_y as f32, w.h(), 0.0, y) as i32 - 8,
		)
	};

	let mut dst_rect = Rect::new(0, 0, 8, 8);
	for a in w.atoms() {
		let (x, y) = pix(a.p[0], a.p[1]);
		dst_rect.set_x(x);
		dst_rect.set_y(y);
		canvas.copy_ex(&texture, None, Some(dst_rect), 0.0, None, false, false)?;
	}

	canvas.present();
	Ok(())
}

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;
type Error = Box<dyn error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn init_window() -> Result<(Canvas, EventPump)> {
	// ---- init window, canvas
	let context = sdl2::init()?;
	let video = context.video()?;
	let window = video
		.window("atoms", 880, 660)
		.position_centered()
		.build()?;
	match window.into_canvas().present_vsync().build() {
		Ok(c) => Ok((c, context.event_pump()?)),
		Err(e) => Err(Box::new(e)),
	}
}
