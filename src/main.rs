use glib::clone;
use gtk::{
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::{BoxExt, GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, DropDown, Switch,
};

const APP_ID: &str = "io.github.aerphanas";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let operator_choose = DropDown::from_strings(&["AND", "OR"]);
    let gtk_box: Vec<gtk::Box> = (0..1)
        .map(|_| {
            let switch: Vec<Switch> = (0..3)
                .map(|_| {
                    let switch_child = Switch::new();
                    switch_child.connect_state_notify(clone!(@weak switch_child => move |s| {
                     println!("{}", s.state())
                    }));
                    switch_child
                })
                .collect();
            let boxs = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .build();
            switch.last().unwrap().set_sensitive(false);
            switch.iter().for_each(|switch| boxs.append(switch));
            boxs
        })
        .collect();
    let box_parent = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .margin_bottom(10)
        .build();

    box_parent.append(&operator_choose);
    gtk_box.iter().for_each(|boxs| box_parent.append(boxs));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Bits")
        .default_width(160)
        .child(&box_parent)
        .build();

    window.present();
}
