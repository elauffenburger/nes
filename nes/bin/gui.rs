use libnes::ppu::nametable::NAMETABLE_DIMS;
use std::cell::RefCell;
use std::rc::Rc;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use libnes::cpu::Cpu;
use libnes::nes::Nes;
use libnes::ppu::Ppu;

struct AppDebugState {
    nametable_tile_index: u16,
    print_debug_info: bool,
}

struct App {
    gl: GlGraphics,
    nes: Rc<RefCell<Nes>>,

    debug: AppDebugState,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let mut nes = self.nes.borrow_mut();
        let mut ppu = nes.get_ppu();

        let nametable_draw_tile_index = self.debug.nametable_tile_index;

        let print_debug_info = self.debug.print_debug_info;
        if print_debug_info {
            self.debug.print_debug_info = false;
        }

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            let nametable = ppu.borrow().get_active_nametable();
            let pattern_table = ppu.borrow_mut().get_active_pattern_table();

            if print_debug_info {
                println!("nametable:\n{:?}\n", &nametable);

                // print header for data
                for col in 0..NAMETABLE_DIMS[1] {
                    print!("{:#02}\t", col);
                }

                println!();
            }

            let mut index = 0;

            for row in 0..NAMETABLE_DIMS[0] {
                for col in 0..NAMETABLE_DIMS[1] {
                    let tile = nametable.get_tile_at_loc(row, col, &pattern_table);

                    // TODO: actually use palette for stuff
                    let colors = tile.pattern_table_tile.get_color_indices();

                    if print_debug_info {
                        if col == 0 && index != 0 {
                            println!("");
                        }

                        print!("{:#02x}({})\t", tile.pattern_table_tile_index, tile.index);
                    }

                    const TILE_SIZE: f64 = 16.0;
                    let tile_x_offset = col as f64 * TILE_SIZE;
                    let tile_y_offset = row as f64 * TILE_SIZE;

                    for (i, color) in colors.iter().enumerate() {
                        let row = (i / 8) as f64;
                        let col = (i % 8) as f64;

                        let pixel_size = TILE_SIZE / 8.0;

                        rectangle(
                            match color {
                                0 => BLACK,
                                _ => GREEN,
                            },
                            rectangle::square(0.0, 0.0, pixel_size),
                            c.transform.trans(
                                tile_x_offset + (col * pixel_size),
                                tile_y_offset + (row * pixel_size),
                            ),
                            gl,
                        );
                    }

                    index += 1;
                }
            }

            if print_debug_info {
                println!();
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let mut nes = self.nes.borrow_mut();
        nes.tick();
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

    let mut app = App {
        gl: GlGraphics::new(opengl),
        nes,
        debug: AppDebugState {
            nametable_tile_index: 140,
            print_debug_info: false,
        },
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Right => {
                    app.debug.nametable_tile_index += 1;
                }
                Key::Left => {
                    app.debug.nametable_tile_index -= 1;
                }
                Key::Return => {
                    app.debug.print_debug_info = true;
                }
                _ => {}
            }
        }
    }
}
