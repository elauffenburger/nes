use gio::prelude::*;
use gtk::prelude::*;

use gtk::*;

use libnes::cpu::Cpu;

pub fn start_gui(cpu: &mut Cpu) {
    let application = gtk::Application::new("com.nes.main", Default::default()).expect("gtk init failed");

    application.connect_activate(|app| {
        build_gui(app);
    });

    application.run(&[]);
}

fn build_gui(app: &gtk::Application) {
    let window = ApplicationWindow::new(app);
    window.set_title("Hello, world!");
    window.set_default_size(640, 480);

    let gl_area = GLArea::new();
    gl_area.connect_render(|a, ctx| {
        ctx.make_current();

        return Inhibit(false);
    });

    window.add(&gl_area);

    window.show_all();

    gtk::main();
}