use edda_gui_util::{
    log,
    pop_ups::{DialogLevel, message},
};
use gdk4;
use gdk4::Display;
use gtk4::glib::{ExitCode, clone};
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Button, CssProvider, HeaderBar, Label, StyleContext, TextView,
};
use gtk4::{ScrolledWindow, TextBuffer, WrapMode};

mod menus;

const APP_ID: &str = "com.cmgsk.edda";
fn main() -> ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(ui_builder);

    println!("Serving UI...");
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    log!(WAR, "Loading CSS on gtk is wrapped in unsafe code...");
    provider.load_from_path("./assets/gtk.css");
    log!(INF, "Valid CSS path.");

    if let Some(screen) = Display::default() {
        gtk4::style_context_add_provider_for_display(
            &screen,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_USER,
        );
        log!(INF, "Loaded default theme.")
    } else {
        log!(WAR, "The CSS gtk style could not be applied.")
    }
}

fn ui_builder(app: &Application) {
    // --- Button definitions ---
    let b_open = Button::builder()
        .label("Open")
        .tooltip_text("Open a file")
        .build();
    let b_save = Button::builder()
        .label("Save")
        .tooltip_text("Save the file")
        .build();
    let b_save_as = Button::builder()
        .label("Save as...")
        .tooltip_text("Save the file as...")
        .build();

    // --- Header bar definition ---
    let header_bar = HeaderBar::builder()
        .title_widget(&Label::new(Some("Edda")))
        .show_title_buttons(true)
        .build();

    // --- Header bar packing ---
    header_bar.pack_start(&b_open);
    header_bar.pack_start(&b_save);
    header_bar.pack_start(&b_save_as);

    // --- Text viewer ---
    let text_view = TextView::builder()
        .editable(true)
        .cursor_visible(true)
        .wrap_mode(WrapMode::WordChar)
        .pixels_above_lines(10)
        .pixels_below_lines(10)
        .pixels_inside_wrap(10)
        .left_margin(20)
        .right_margin(20)
        .build();

    // --- Text Buffer ---
    // Holds the actual text within the text view
    let text_buffer = TextBuffer::builder().build();
    text_view.set_buffer(Some(&text_buffer));

    // --- Scrolled window ---
    // Makes the page scrollable
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Never)
        .child(&text_view)
        .build();

    // --- Main window layout ---
    let main_window = ApplicationWindow::builder()
        .application(app)
        .title(APP_ID)
        .default_height(1920)
        .default_height(1080)
        .build();

    main_window.set_titlebar(Some(&header_bar));
    main_window.set_child(Some(&scrolled_window));

    let buf_clone = text_buffer.clone();
    b_save.connect_clicked(move |_| {
        let g = buf_clone.text(&buf_clone.start_iter(), &buf_clone.end_iter(), false);
        println!("Clicked save for: {}", g);
    });
    b_save_as.connect_clicked(clone!(
        #[weak]
        main_window,
        move |_| {
            message(&main_window, DialogLevel::Info, "Unimplemented", false);
            println!("Unimplemented");
        }
    ));
    b_open.connect_clicked(clone!(
        #[weak]
        main_window,
        move |_| {
            println!("Clicked open button!");
            menus::file::external::file_chooser(
                &main_window,
                "Open file...",
                gtk4::FileChooserAction::Open,
            );
        }
    ));

    println!("Main window ready to present.");
    main_window.present()
}
