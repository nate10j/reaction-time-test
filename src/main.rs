use std::time::Instant;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, SimpleComponent};

struct Model {
    timer: Option<Instant>,
    time: f32,
}

#[derive(Debug)]
enum Event {
    StartTime,
    StopTime
}

#[relm4::component]
impl SimpleComponent for Model {
    type Init = ();
    type Input = Event;
    type Output =  ();

    view! {
        gtk::Window {
            set_title: Some("Better Reaction Time"),
            set_default_width: 800,
            set_default_height: 500,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                gtk::Button {
                    set_label: "Start",
                    connect_clicked => Event::StartTime
                },
                gtk::Button {
                set_label: "Stop",
                connect_clicked => Event::StopTime
                },
                gtk::Label {
                    #[watch]
                    set_label: &format!("{} ms", (model.time * 1000.0).floor()),
                }
            }
        }
    }

    fn init(time: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = Model { timer: None, time: 0.0 };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, event: Self::Input, _sender: ComponentSender<Self>) {
        match event {
            Event::StartTime => {
                self.timer = Some(Instant::now());
            }
            Event::StopTime => {
                self.time = self.timer.unwrap().elapsed().as_secs_f32();
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("com.github.nate10j.BetterReactionTime");
    app.run::<Model>(());
}
