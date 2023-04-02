use std::{cell::Cell, rc::Rc};
use glib::clone;
use gtk::{
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::{BoxExt, ButtonExt, GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, Button, DropDown, Switch,
};

const APP_ID: &str = "io.github.aerphanas";

fn main() {
    let app: Application = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let operator_choose: DropDown = DropDown::from_strings(&["AND", "OR", "XOR"]);
    let button_ressult: Button = Button::builder().label("Callculate").build();
    let left_val: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let right_val: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    let gtk_box: Vec<gtk::Box> = (0..1)
        .map(|_| {
            let switch: Vec<Switch> = (0..3)
                .map(|num| {
                    let switch_child = Switch::new();
                    switch_child.connect_state_notify(
                        clone!(@weak right_val, @weak left_val =>
                            move |switch_child| {
                                match num {
                                    0 => left_val.set(switch_child.state()),
                                    1 => right_val.set(switch_child.state()),
                                    _ => ()
                                }
                            })
                        );
                    switch_child
                })
                .collect();

            button_ressult.connect_clicked(
                clone!(@strong right_val, @strong left_val, @strong switch, @weak operator_choose =>
                    move |_| {
                        let result: bool = match operator_choose.selected() {
                            0 => right_val.get() && left_val.get(),
                            1 => right_val.get() || left_val.get(),
                            2 => right_val.get() ^ left_val.get(),
                            _ => false
                        };
                        switch
                            .last()
                            .unwrap()
                            .set_state(result)
                    }
                ),
            );
            let boxs: gtk::Box = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .build();
            switch.last().unwrap().set_sensitive(false);
            switch.iter().for_each(|switch| boxs.append(switch));
            boxs
        })
        .collect();

    let box_parent: gtk::Box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .margin_bottom(10)
        .build();

    box_parent.append(&operator_choose);
    box_parent.append(&button_ressult);
    gtk_box.iter().for_each(|boxs| box_parent.append(boxs));

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title("Bits")
        .default_width(160)
        .child(&box_parent)
        .build();

    window.present();
}

