use std::time::Instant;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, SimpleComponent, RelmWidgetExt};

struct Model {
    timer: Option<Instant>,
    reaction_test_state: ReactionTestState,
    time: f32,
}

// State is what it is showing e.g. Start has not started timer, but the button says start
#[derive(PartialEq)]
enum ReactionTestState {
    Start,
    Waiting,
    Stop,
    Stopped
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
                #[watch]
                set_class_active: ("waiting", model.reaction_test_state == ReactionTestState::Waiting),
                #[watch]
                set_class_active: ("stop", model.reaction_test_state == ReactionTestState::Stopped),
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

    fn init(_params: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = Model {
            timer: None,
            reaction_test_state: ReactionTestState::Start,
            time: 0.0,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, event: Self::Input, _sender: ComponentSender<Self>) {
        match event {
            Event::StartTime => {
                self.reaction_test_state = ReactionTestState::Waiting;
                self.timer = Some(Instant::now());
            }
            Event::StopTime => {
                self.reaction_test_state = ReactionTestState::Stopped;
                self.time = self.timer.unwrap().elapsed().as_secs_f32();
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("com.github.nate10j.BetterReactionTime");
    let _ = relm4::set_global_css_from_file(std::path::Path::new("src/styles.css"));
    app.run::<Model>(());
}
