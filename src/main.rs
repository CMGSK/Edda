use gtk4::glib::{ExitCode, clone};
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, HeaderBar, Label, TextView};
use gtk4::{ScrolledWindow, TextBuffer, WrapMode};

mod menus;

const APP_NAME: &str = "Edda - Office writer";
fn main() -> ExitCode {
    let app = Application::builder().application_id(APP_NAME).build();
    app.connect_activate(ui_builder);
    app.run()
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
        .title(APP_NAME)
        .default_height(1920)
        .default_height(1080)
        .build();

    main_window.set_titlebar(Some(&header_bar));
    main_window.set_child(Some(&scrolled_window));

    let buf_clone = text_buffer.clone();
    b_save.connect_clicked(move |_| {
        println!("Clicked save for: {buf_clone:?}");
    });
    b_save_as.connect_clicked(move |_| {
        println!("Unimplemented");
    });
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
}
