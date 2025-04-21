use edda_gui_util::pop_ups::DialogLevel;
use gtk4::prelude::*;
use gtk4::{
    Adjustment, ApplicationWindow, Box, Button, FontButton, Orientation, Separator, SpinButton,
};

pub fn create_edition_toolbar() -> gtk4::Box {
    // --- Toolbar definition ---
    let toolbar = Box::new(gtk4::Orientation::Horizontal, 5);
    toolbar.set_spacing(5);

    // --- First block components ---
    let b_undo = Button::from_icon_name("go-previous-symbolic");
    b_undo.set_tooltip_text(Some("Undo"));

    let b_redo = Button::from_icon_name("go-next-symbolic");
    b_redo.set_tooltip_text(Some("Redo"));

    let b_prnt = Button::from_icon_name("document-print-symbolic");
    b_prnt.set_tooltip_text(Some("Print..."));

    toolbar.append(&b_undo);
    toolbar.append(&b_redo);
    toolbar.append(&b_prnt);
    toolbar.append(&Separator::new(Orientation::Vertical));

    // --- Second block components ---
    let b_font = FontButton::new();
    b_font.set_tooltip_text(Some("Select font family"));

    let t_size = Adjustment::new(12.0, 6.0, 72.0, 1.0, 5.0, 0.0);
    let b_spin = SpinButton::new(Some(&t_size), 1.0, 0);
    b_spin.set_tooltip_text(Some("Font size (pt)"));
    b_spin.set_numeric(true);
    b_spin.set_width_chars(3);

    toolbar.append(&b_font);
    toolbar.append(&b_spin);
    toolbar.append(&Separator::new(Orientation::Vertical));

    // --- Third block components ---
    let b_bold = Button::from_icon_name("format-text-bold-symbolic");
    b_bold.set_tooltip_text(Some("Bold"));

    let b_ita = Button::from_icon_name("format-text-italic-symbolic");
    b_ita.set_tooltip_text(Some("Italic"));

    let b_under = Button::from_icon_name("format-text-underline-symbolic");
    b_under.set_tooltip_text(Some("Underline"));

    let b_color = Button::from_icon_name("color-select-symbolic");
    b_color.set_tooltip_text(Some("Font color"));

    let b_hlight = Button::from_icon_name("edit-find-replace-symbolic");
    b_hlight.set_tooltip_text(Some("Highlight color"));

    toolbar.append(&b_bold);
    toolbar.append(&b_ita);
    toolbar.append(&b_under);
    toolbar.append(&b_color);
    toolbar.append(&b_hlight);

    // --- Button callback definitions ---
    b_undo.connect_clicked(|b| {
        if let Some(r) = b.root() {
            match r.downcast::<ApplicationWindow>() {
                Ok(window) => edda_gui_util::pop_ups::message(
                    &window,
                    DialogLevel::Info,
                    "This is not yet implemented",
                    false,
                ),
                Err(_) => println!("This is yet to implement."),
            }
        }
    });

    toolbar
}
