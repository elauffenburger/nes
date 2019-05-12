use std::cell::RefCell;
use std::rc::Rc;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use libnes::nes::Nes;

struct App {
    gl: GlGraphics,
    nes: Rc<RefCell<Nes>>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
        });

        // DEBUG
        // let ppu = self.nes.borrow_mut().get_ppu();
        // let tile = &ppu.borrow_mut().get_pattern_tables()[0].get_tile_at_index(1);

        // println!("ppu tile 0: {:?}", tile);
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.nes.borrow_mut().clock();
    }
}

pub fn start_gui(nes: Rc<RefCell<Nes>>) {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("nes", [640, 480])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Start NES up -- this should probably be controlled
    // somewhere else eventually
    nes.borrow_mut().start();

    // DEBUG
    let ppu = nes.borrow_mut().get_ppu();
    let table = &ppu.borrow_mut().get_pattern_tables()[0];

    // for (i, byte) in mem.iter().enumerate() {
    //     if i % 16 == 0 {
    //         println!("");
    //     }

    //     print!("{:02x}\t", byte);
    // }

    println!("ppu pattern table one:\n{:?}", table);

    let mut app = App {
        gl: GlGraphics::new(opengl),
        nes,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
