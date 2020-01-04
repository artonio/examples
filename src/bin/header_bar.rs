//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and
//! position, how to add a `button` to this `window` and how to connect signals with
//! actions.

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Header bar Demo");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(600, 200);

    let hb = gtk::HeaderBar::new();
    hb.set_show_close_button(true);
    hb.set_title(Some("HeaderBar Example"));
    window.set_titlebar(Some(&hb));

    let button = gtk::Button::new();
    let icon = gio::ThemedIcon::new("mail-send-receive-symbolic");
    let image = gtk::Image::new_from_gicon(&icon, gtk::IconSize::Button);
    button.add(&image);
    hb.pack_end(&button);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let style_context = hbox.get_style_context();
    // https://developer.gnome.org/gtk3/stable/GtkStyleContext.html#GTK-STYLE-CLASS-LINKED:CAPS
    style_context.add_class("linked");

    let button_left = gtk::Button::new_with_label("<");
    let bl_style_ctx = button_left.get_style_context();
    // Gtk Arrow not implemented?
    // button.add(Gtk.Arrow(Gtk.ArrowType.LEFT, Gtk.ShadowType.NONE))
    hbox.add(&button_left);

    let button_right = gtk::Button::new_with_label(">");
    hbox.add(&button_right);

    hb.pack_start(&hbox);
    let text_view = gtk::TextView::new();
    window.add(&text_view);

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.headerbar"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
