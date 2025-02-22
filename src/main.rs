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
    Fail,
    Stop,
    Stopped
}

#[derive(Debug)]
enum Event {
    Click,
    Stop,
}

#[relm4::component]
impl SimpleComponent for Model {
    type Init = ();
    type Input = Event;
    type Output =  ();


    // fix indentation here; could be lsp issue
    view! {
        gtk::Window {
            set_title: Some("Better Reaction Time"),
            set_default_width: 800,
            set_default_height: 500,

            gtk::Button {
                #[watch]
                set_class_active: ("waiting", model.reaction_test_state == ReactionTestState::Waiting),
                #[watch]
                set_class_active: ("stop", model.reaction_test_state == ReactionTestState::Stop),


                match model.reaction_test_state {
                ReactionTestState::Start => {
                    gtk::Label {
                        set_label: "Start"
                    }
                }
                ReactionTestState::Waiting => {
                    gtk::Label {
                        set_label: "Waiting"
                    }
                }
                ReactionTestState::Stopped => {
                    gtk::Label {
                        set_label: "Click to reset"
                    }
                }
                _ => {
                gtk::Label {
                    set_label: ""
                }
            }

        },

        connect_clicked => Event::Click,
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

    fn update(&mut self, event: Self::Input, sender: ComponentSender<Self>) {
        match event {
            Event::Click => {
                match self.reaction_test_state {
                    ReactionTestState::Start => {
                        // started
                        self.reaction_test_state = ReactionTestState::Waiting;
                        tokio::spawn(async move {
                            wait_for_stop(sender).await;
                        });
                    }
                    ReactionTestState::Waiting => {
                        // clicked during waiting, too early
                        self.reaction_test_state = ReactionTestState::Fail;
                    }
                    ReactionTestState::Fail => {
                        self.reaction_test_state = ReactionTestState::Stopped;
                    }
                    ReactionTestState::Stop => {
                        // perfect time to stop
                        self.reaction_test_state = ReactionTestState::Stopped;
                    }
                    ReactionTestState::Stopped => {
                        self.reaction_test_state = ReactionTestState::Start;
                    }
                }
                self.timer = Some(Instant::now());
            }
            Event::Stop => {
                self.reaction_test_state = ReactionTestState::Stop;
            }
        }
    }
}

async fn wait_for_stop(sender: ComponentSender<Model>) {
    tokio::time::sleep(std::time::Duration::new(2, 0)).await;
    sender.input(Event::Stop);
}

#[tokio::main]
async fn main() {
    let app = RelmApp::new("com.github.nate10j.BetterReactionTime");
    let _ = relm4::set_global_css_from_file(std::path::Path::new("src/styles.css"));
    app.run::<Model>(());
}
