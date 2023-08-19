
use std::sync::{Arc, Mutex};
use std::{time, thread};

use gtk::{prelude::*, Window};
use gtk::{ WindowType};
use rdev::{Key, EventType, simulate};
// fn on_activate(application: &gtk::Application) {
//     // … create a new window …
//     let window = gtk::ApplicationWindow::new(application);
//     // … with a button in it …
//     let button = gtk::Button::with_label("Hello World!");
//     // … which closes the window when clicked
//     button.connect_clicked(glib::clone!(@weak window => move |_| window.close()));
//     window.set_child(Some(&button));
//     window.present();
// }
fn resize_window(window: &Window,vbox:&gtk::Box) {
    vbox.set_resize_mode(gtk::ResizeMode::Immediate);
    window.set_resize_mode(gtk::ResizeMode::Immediate);
    
     let allocation1 = vbox.allocation();
    let width1 = allocation1.width();
    let height1 = allocation1.height();
    vbox.set_size_request(width1, height1);
    vbox.queue_resize(); // Trigger a resize event

    let allocation = window.allocation();
    let width = allocation.width();
    let height = allocation.height();
    window.set_size_request(width, height);
    window.queue_resize(); // Trigger a resize event

}

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
    let button3 = gtk::Button::with_label("Fullscreen");
    button3.set_margin(20);
    button3.connect_clicked(|_| {
        fullscreen();

    });
    let button2 = gtk::Button::with_label("Quit");
    button2.set_margin(20);
    button2.connect_clicked(|_| {
        gtk::main_quit(); // Close the window and quit the GTK main event loop

    });
    
  // Create the toggle button
  let toggle_button = gtk::Button::with_label("Toggle");
  let mut showornot=true;
  let arco=Arc::new(Mutex::new(showornot));
  let arco_clone=arco.clone();
  

  let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
//   vbox.set_no_show_all(true); 
  vbox.add(&button);
  vbox.add(&button2);
  vbox.add(&button3);
  vbox.add(&toggle_button);
//   vbox.set_resize_mode(gtk::ResizeMode::Parent);
//   window.set_resize_mode(gtk::ResizeMode::Parent);

  window.add(&vbox);
  toggle_button.connect_clicked(glib::clone!(@weak vbox, @weak button,@weak button2,@weak button3, @weak window => move |_| {
    let cv=*arco.lock().unwrap();
    if(cv){
        button.hide();
        button2.hide();
        button3.hide();
    vbox.set_no_show_all(true); 

    }
    else{
        

        button.show();
        button2.show();
        button3.show();
    vbox.set_no_show_all(true); 

    }
    
    let mut data=arco_clone.lock().unwrap();
    *data=!cv;
    resize_window(&window,&vbox)
    // gtk::main_quit(); // Close the window and quit the GTK main event loop

}));
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
fn fullscreen(){
    send(&EventType::KeyPress(Key::Alt));
    send(&EventType::KeyPress(Key::F11));
    send(&EventType::KeyRelease(Key::F11));
    send(&EventType::KeyRelease(Key::Alt));
}

fn godesktop(){
    send(&EventType::KeyPress(Key::MetaLeft));
    send(&EventType::KeyPress(Key::KeyD));
    send(&EventType::KeyRelease(Key::KeyD));
    send(&EventType::KeyRelease(Key::MetaLeft));
}