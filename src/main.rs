use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, SubCommand,
};
use gtk::{prelude::*, WindowType};
use std::sync::{Arc, Mutex};

fn read_char(message: &str, markup: &str) -> char {
    gtk::init().expect("Failed to initialize gtk");

    let window = gtk::Window::new(WindowType::Popup);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let selected_char = Arc::new(Mutex::new(None));

    let selected_char_clone = selected_char.clone();
    window.connect_key_press_event(move |_, event_key| {
        let ch = event_key.get_keyval().to_unicode().unwrap();

        *selected_char_clone.lock().unwrap() = Some(ch);

        gtk::main_quit();
        Inhibit(false)
    });

    gtk_layer_shell::init_for_window(&window);
    gtk_layer_shell::set_keyboard_interactivity(&window, true);
    gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Overlay);

    let label = gtk::Label::new(None);
    label.set_markup(&markup.replace("{}", message));
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
    // Init clap arguments parser
    let args = app_from_crate!()
        .arg(
            Arg::with_name("markup")
                .long("markup")
                .short("m")
                .help("Pango markup used to format displayed message")
                .default_value("<span font_desc='15'>{}</span>"),
        )
        .subcommand(SubCommand::with_name("mark").about("Mark current window"))
        .subcommand(SubCommand::with_name("goto").about("Jump to a marked window"))
        .get_matches();
    // unwrap() never panics because "markup" has a default value
    let markup = args.value_of("markup").unwrap();

    // Open swayipc connection
    let mut sway_conn =
        swayipc::Connection::new().expect("Failed to establish a connection to sway ipc");

    if args.subcommand_matches("mark").is_some() {
        let ch = read_char("Enter mark: ", markup);
        println!("Creating mark {}", ch);
        sway_conn
            .run_command(format!("mark {}", ch))
            .expect("Failed to run sway command");
    } else if args.subcommand_matches("goto").is_some() {
        let ch = read_char("Go to mark: ", markup);
        println!("Navigating to mark {}", ch);
        sway_conn
            .run_command(format!("[con_mark={}] focus", ch))
            .expect("Failed to run sway command");
    } else {
        eprintln!("Missing argument! Specify either \"mark\" or \"goto\"");
    }
}
