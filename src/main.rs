
use std::{time, thread};

use gtk::prelude::*;
use gtk::{ WindowType};
use rdev::{Key, EventType, simulate};
    
fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = gtk::Window::new(WindowType::Toplevel);
    window.set_title("Unfocusable Window");
    window.set_decorated(false);
    // window.set_default_size(480, 480);
    window.set_keep_above(true);
    window.set_skip_taskbar_hint(true);
    window.set_skip_pager_hint(true);
    window.set_accept_focus(false);

    let button = gtk::Button::with_label("A");
    button.set_margin(20);
    button.connect_clicked(|_| {
        pressandrelease(Key::KeyA);

    });
    let button2 = gtk::Button::with_label("Quit");
    button2.set_margin(20);
    button2.connect_clicked(|_| {
        gtk::main_quit(); // Close the window and quit the GTK main event loop

    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.add(&button);
    vbox.add(&button2);
    window.add(&vbox);

    window.show_all();
    window.connect_button_press_event(move |_window, event| {
        let coord=event.root();
        if event.button() == 1 {
            _window.begin_move_drag(
                event.button() as i32,
                coord.0 as i32,
               coord.1 as i32,
                event.time()
            );
        }
        Inhibit(false)
    });
    gtk::main();
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}
fn pressandrelease(key:Key){
    send(&EventType::KeyPress(key));
    send(&EventType::KeyRelease(key));
}