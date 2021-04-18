extern crate gl;
extern crate sdl2;

use std::ffi::{CStr, CString};
mod tmgl;

macro_rules! file_name_and_content {
    ($a:tt)=>{
        ($a, include_str!($a))
    };

}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let a = file_name_and_content!("triangle.frag");

    tmgl::shader_from_file(("triangle.frag", include_str!("triangle.frag")));

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        tmgl::clear(0.0, 0.0, 0.0, 0.0);

        window.gl_swap_window();
    }
}
