use crate::log;
use glib::clone;
use glib::prelude::*;
use gtk4::prelude::{DialogExt, GtkWindowExt};
use gtk4::{ButtonsType, DialogFlags, MessageDialog, MessageType, ResponseType, Window};
use std::cell::RefCell;
use std::rc::Rc;

pub enum DialogLevel {
    Info,
    Warning,
    Error,
    Critical, // For debugging purposes on displaying Err()
}

pub fn message(parent: &impl IsA<Window>, level: DialogLevel, message: &str, sticky: bool) {
    let level = match level {
        DialogLevel::Info => MessageType::Info,
        DialogLevel::Warning => MessageType::Warning,
        DialogLevel::Error => MessageType::Error,
        DialogLevel::Critical => MessageType::Error,
    };

    match level {
        MessageType::Info => log!(INF, message),
        MessageType::Warning => log!(WAR, message),
        MessageType::Error => log!(ERR, message),
        _ => log!(CRT, "Unexpected MessageType in message popup"),
    };

    let flags = match sticky {
        true => DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT, // Restrict message lifetime to its parent's (and modal)
        false => DialogFlags::MODAL, //Acknowledge message before proceeding
    };

    let buttons = match level {
        MessageType::Question => ButtonsType::YesNo,
        _ => ButtonsType::Ok,
    };

    let dialog = MessageDialog::new(Some(parent), flags, level, buttons, message);

    dialog.connect_response(move |dialog, _t| {
        dialog.destroy();
    });

    dialog.present();
}

pub fn question<F>(parent: &impl IsA<Window>, message: &str, callback: F)
where
    F: FnOnce(bool) + 'static,
{
    let flags = DialogFlags::MODAL | DialogFlags::DESTROY_WITH_PARENT;
    let level = MessageType::Question;
    let buttons = ButtonsType::YesNo;

    let dialog = MessageDialog::new(Some(parent), flags, level, buttons, message);

    let holder = Rc::new(RefCell::new(Some(callback)));

    dialog.connect_response(clone!(
        #[strong]
        holder,
        move |dialog, response| {
            if let Some(callback) = holder.borrow_mut().take() {
                callback(response == ResponseType::Yes);
            }
            dialog.destroy();
        }
    ));

    dialog.present();
}
