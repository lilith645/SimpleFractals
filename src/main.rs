/*
 * main.rs
 * This file is part of SimpleFractals
 *
 * Copyright (C) 2017 - Akuma
 *
 * SimpleFractals is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * SimpleFractals is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with SimpleFractals. If not, see <http://www.gnu.org/licenses/>.
 */

#[macro_use]
extern crate glium;

use std::time::Instant;
use glium::glutin::ElementState;
use glium::{DisplayBuild, Surface};
use glium::glutin::{Event, VirtualKeyCode, WindowBuilder};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
  println!("Hello, world!");
    
  let display = WindowBuilder::new().with_title("Simple Fractals")
                                      .with_dimensions(1024, 768)
                                      .build_glium().unwrap();
                                      
  let shader = glium::Program::from_source(&display, include_str!("shader.vs"),
                                                     include_str!("shader.frag"), 
                                                     None).unwrap();
                                                        
  let vertices = [
    // Top-left corner
    Vertex{ position: [-1.0,  1.0] },
    Vertex{ position: [ 1.0,  1.0] },
    Vertex{ position: [-1.0, -1.0] },

    // Bottom-right corner
    Vertex { position: [-1.0, -1.0] },
    Vertex { position: [ 1.0,  1.0] },
    Vertex { position: [ 1.0, -1.0] },
  ];
  
  let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
  
  let mut zoom: f32 = 0.0;
  let mut offset: f32 = 2.001;
  let mut speed: f32 = 800.0;
  
  // Fractal Type
  // 0 - MandelbrotSet
  // 1 - SierpinskiCarpet
  // 2 - julia fractals
  let mut fractal_type: i32 = 2;
  
  // start timer for delta time
  let mut last_time = Instant::now();
  
  loop {
    let delta_time = last_time.elapsed().subsec_nanos() as f32 / 1000000000.0;
    last_time = Instant::now();
    
    let mut render = display.draw();
    
    render.draw(&vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &shader,
                    &uniform! { zoom: zoom, offset: offset, fractal_type: fractal_type },
                    &Default::default()).unwrap();
                    
    render.finish().unwrap();
    
    for event in display.poll_events() {
      match event {
        // the window has been closed by the user:
        Event::Closed => return,
        Event::KeyboardInput(_, _, Some(VirtualKeyCode::W)) => offset+=(3.0 - speed/800.0)*delta_time,
        Event::KeyboardInput(_, _, Some(VirtualKeyCode::S)) => offset-=(3.0 - speed/800.0)*delta_time,
        Event::KeyboardInput(_, _, Some(VirtualKeyCode::D)) => { zoom+=speed*delta_time; speed+=speed/800.0; },
        Event::KeyboardInput(_, _, Some(VirtualKeyCode::A)) => { zoom-=speed*delta_time; speed-=speed/800.0; },
        Event::KeyboardInput(ElementState::Released, _,Some(VirtualKeyCode::C)) => { if fractal_type < 2 { 
                                                                                       fractal_type+=1; 
                                                                                     } else {
                                                                                       fractal_type = 0;
                                                                                     }
                                                                                     zoom = 0.0;
                                                                                     offset = 2.001;
                                                                                     speed = 800.0;
                                                                                   },
        // Quit on Esc: 
        Event::KeyboardInput(_ , _, Some(VirtualKeyCode::Escape)) => return,
        _ => ()
      }
    }
  }
}
