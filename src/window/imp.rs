use std::{cell::Cell, rc::Rc};

use gtk::{
    glib::{self, clone, subclass::InitializingObject},
    subclass::{
        prelude::{ApplicationWindowImpl, ObjectImpl, ObjectImplExt, ObjectSubclass},
        widget::{
            CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetClassSubclassExt,
            WidgetImpl,
        },
        window::WindowImpl,
    },
    traits::ButtonExt,
    Button, CompositeTemplate, DropDown, Switch, TemplateChild,
};

#[derive(Default, CompositeTemplate)]
#[template(resource = "/io/github/aerphanas/window.ui")]
pub struct Window {
    #[template_child]
    pub drop_down_option: TemplateChild<DropDown>,

    #[template_child]
    pub calculate_button: TemplateChild<Button>,

    #[template_child]
    pub left_switch: TemplateChild<Switch>,

    #[template_child]
    pub right_switch: TemplateChild<Switch>,

    #[template_child]
    pub result_switch: TemplateChild<Switch>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "BitsAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let left_val = Rc::new(Cell::new(false));
        let right_val = Rc::new(Cell::new(false));
        let operator_choose = self.drop_down_option.get();
        let ress_switch = self.result_switch.get();

        self.calculate_button.connect_clicked(clone!(
            @strong right_val,
            @strong left_val,
            @weak operator_choose,
            @weak ress_switch => move |_| {
                let result = match operator_choose.selected() {
                    0 => right_val.get() && left_val.get(),
                    1 => right_val.get() || left_val.get(),
                    2 => right_val.get() ^ left_val.get(),
                    _ => false
                };
                ress_switch.set_active(result);
            }
        ));

        self.left_switch
            .connect_state_notify(clone!(@weak left_val =>
            move |switch_child| {
                    left_val.set(switch_child.state());
            }));

        self.right_switch
            .connect_state_notify(clone!(@weak right_val =>
            move |switch_child| {
                    right_val.set(switch_child.state());
            }));
    }
}

impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
