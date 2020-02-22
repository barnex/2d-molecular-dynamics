// extern crate image;
//
// use crate::prelude::*;
// use std::io;
//
// pub fn render(w: &World, num_pix: u32, file: &str) -> io::Result<()> {
// 	let pix_x = num_pix as i32;
// 	let pix_y = ((num_pix as f32) * (w.h / w.w)) as i32;
// 	let mut img = image::ImageBuffer::new(pix_x as u32, pix_y as u32);
//
// 	let r = (0.6 / w.w * num_pix as f32) as i32;
//
// 	let bg_color = image::Rgb([255, 255, 255]);
// 	let atom_color = image::Rgb([10, 10, 255]);
// 	for (_, _, pixel) in img.enumerate_pixels_mut() {
// 		*pixel = bg_color;
// 	}
//
// 	let pix = |x, y| {
// 		(
// 			linterp(-w.w / 2.0, 0.0, w.w / 2.0, pix_x as f32, x) as i32,
// 			linterp(-w.h / 2.0, pix_y as f32, w.h / 2.0, 0.0, y) as i32,
// 		)
// 	};
//
// 	for a in &w.atoms {
// 		let (i, j) = pix(a.p[0], a.p[1]);
//
// 		if i >= 0 && i < pix_x && j >= 0 && j < pix_y {
// 			//draw_atom(&mut img, i, j);
// 			{
// 				let w = img.width() as i32;
// 				let h = img.height() as i32;
//
// 				for di in -r..r {
// 					for dj in -r..r {
// 						let i2 = i + di;
// 						let j2 = j + dj;
// 						if i2 >= 0 && i2 < w && j2 >= 0 && j2 < h {
// 							if (vec2(i2 as f32, j2 as f32) - vec2(i as f32, j as f32)).len()
// 								<= r as f32 - 0.5
// 							{
// 								img.put_pixel(i2 as u32, j2 as u32, atom_color);
// 							}
// 						}
// 					}
// 				}
// 			}
// 		}
// 	}
//
// 	img.save(file)
// }
//
// //fn draw_atom(img: &mut image::ImageBuffer, i: i32, j: i32) {
// //	let w = img.width() as i32;
// //	let h = img.height() as i32;
// //	let r = 3;
// //
// //	for di in -r..r {
// //		for dj in -r..r {
// //			let i = i + di;
// //			let j = j + dj;
// //			if i >= 0 && i < w && j >= 0 && j < h {
// //				img.put_pixel(i as u32, j as u32, atom_color);
// //			}
// //		}
// //	}
// //}
//
// // linear interpolation
// // 	x1 -> y1
// // 	x2 -> y2
// // 	x  -> y
// fn linterp(x1: f32, y1: f32, x2: f32, y2: f32, x: f32) -> f32 {
// 	y1 + (y2 - y1) * (x - x1) / (x2 - x1)
// }
//
// //// Note that the v axis points up, while the y axis points down.
// //func IndexToCam(w, h int, ix, iy float64) (u, v float64) {
// //	W := float64(w)
// //	H := float64(h)
// //
// //	if ix < -0.5 || iy < -0.5 || ix > W-0.5 || iy > H-0.5 {
// //		panic(fmt.Sprintf("IndexToCam: pixel index out of range: w=%v, h=%v, x=%v, y=%v",
// //			w, h, ix, iy))
// //	}
// //
// //	u = linterp(-0.5, 0, W-0.5, 1, ix)
// //	v = linterp(-0.5, 0.5+0.5*(H/W), H-0.5, 0.5-0.5*(H/W), iy)
// //	return u, v
// //}
//
