extern crate gio;
extern crate gtk;

use gtk::{prelude::*, WindowType};
use std::env::args;
use std::sync::{Arc, Mutex};

fn read_char(message: &str) -> char {
    gtk::init().expect("Failed to initialize gtk");

    let window = gtk::Window::new(WindowType::Popup);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let selected_char = Arc::new(Mutex::new(None));

    {
        let selected_char = selected_char.clone();
        window.connect_key_press_event(move |_, event_key| {
            let ch = event_key.get_keyval().to_unicode().unwrap();

            *selected_char.lock().unwrap() = Some(ch);

            gtk::main_quit();
            Inhibit(false)
        });
    }

    gtk_layer_shell::init_for_window(&window);

    gtk_layer_shell::set_keyboard_interactivity(&window, true);

    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);

    let label = gtk::Label::new(None);

    label.set_markup(&format!("<span font_desc='15'>{}</span>", message));
    label.set_margin_start(5);
    label.set_margin_end(5);
    label.set_margin_top(5);
    label.set_margin_bottom(5);

    window.add(&label);
    window.show_all();

    gtk::main();

    let selected_char = selected_char.lock().unwrap();
    selected_char.expect("Missing character")
}

fn main() {
    let args: Vec<String> = args().collect();

    let mut sway_conn =
        swayipc::Connection::new().expect("Failed to establish a connection to sway ipc");

    if args.contains(&"mark".to_string()) {
        let ch = read_char("Enter mark: ");

        println!("Creating mark {}", ch);

        sway_conn
            .run_command(format!("mark {}", ch))
            .expect("Failed to run sway command");
    } else if args.contains(&"goto".to_string()) {
        let ch = read_char("Go to mark: ");

        println!("Navigating to mark {}", ch);

        sway_conn
            .run_command(format!("[con_mark={}] focus", ch))
            .expect("Failed to run sway command");
    } else {
        println!("Missing argument! Specify either \"mark\" or \"goto\"");
    }
}
