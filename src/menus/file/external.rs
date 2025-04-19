use edda_gui_util::pop_ups;
use edda_gui_util::pop_ups::DialogLevel;
use gtk4::ApplicationWindow;
use gtk4::prelude::{DialogExt, FileChooserExt, FileExt, GtkWindowExt};

/// This file contains all the necessary helper functions within the GUI application
/// to run actions that require external interactions such as File Explorers for saving,
/// Browsers to open URLs, or Email clients to name some.

/// Interacts with the File explorer of the system
pub fn file_chooser(parent: &ApplicationWindow, title: &str, action: gtk4::FileChooserAction) {
    let chooser = gtk4::FileChooserDialog::new(
        Some(title),
        Some(parent),
        action,
        &[
            ("Cancel", gtk4::ResponseType::Cancel),
            ("Open", gtk4::ResponseType::Accept),
        ],
    );
    chooser.set_modal(true);
    chooser.connect_response(move |dialog, response| {
        if response == gtk4::ResponseType::Accept {
            if let Some(file) = dialog.file() {
                if let Some(buf) = file.path() {
                    println!("Loaded file: {}", buf.display());
                    //TODO: Edda core handler
                } else {
                    println!("Could not find the path");
                    pop_ups::message(dialog, DialogLevel::Error, "Path not found", false);
                }
            } else {
                println!("Could not load the file");
            }
        }
        dialog.destroy();
    });

    chooser.present();
}
