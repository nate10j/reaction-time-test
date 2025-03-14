use std::time::Instant;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, SimpleComponent, RelmWidgetExt};

struct Model {
    timer: Option<Instant>,
    reaction_display: ReactionTimeDisplay,
    time: f32,
}

// State is what it is showing e.g. Start has not started timer, but the button says start
#[derive(PartialEq)]
enum ReactionTimeDisplay {
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
                set_class_active: ("waiting", model.reaction_display == ReactionTimeDisplay::Waiting),
                #[watch]
                set_class_active: ("stop", model.reaction_display == ReactionTimeDisplay::Stop),


                match model.reaction_display {
                ReactionTimeDisplay::Start => {
                    gtk::Label {
                        set_label: "Start"
                    }
                }
                ReactionTimeDisplay::Waiting => {
                    gtk::Label {
                        set_label: "Waiting"
                    }
                }
                ReactionTimeDisplay::Stopped => {
                    gtk::Label {
                        set_label: &format!("{} ms Click to reset", model.time)
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
            reaction_display: ReactionTimeDisplay::Start,
            time: 0.0,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, event: Self::Input, sender: ComponentSender<Self>) {
        match event {
            Event::Click => {
                match self.reaction_display {
                    ReactionTimeDisplay::Start => {
                        // started
                        self.reaction_display = ReactionTimeDisplay::Waiting;
                        tokio::spawn(async move {
                            wait_for_stop(sender).await;
                        });
                    }
                    ReactionTimeDisplay::Waiting => {
                        // clicked during waiting, too early
                        self.reaction_display = ReactionTimeDisplay::Fail;
                    }
                    ReactionTimeDisplay::Fail => {
                        self.reaction_display = ReactionTimeDisplay::Stopped;
                    }
                    ReactionTimeDisplay::Stop => {
                        // after click stop
                        
                        self.time = self.timer.unwrap().elapsed().as_secs_f32();
                        self.reaction_display = ReactionTimeDisplay::Stopped;
                    }
                    ReactionTimeDisplay::Stopped => {
                        self.reaction_display = ReactionTimeDisplay::Start;
                    }
                }
            }

            // this is invoked when time is passed and user has to stop
            // timer is started
            Event::Stop => {
                self.timer = Some(Instant::now());
                self.reaction_display = ReactionTimeDisplay::Stop;
            }
        }
    }
}

async fn wait_for_stop(sender: ComponentSender<Model>) {
    tokio::time::sleep(std::time::Duration::new(2, 0)).await;
    sender.input(Event::Stop);
}

fn main() {
    let app = RelmApp::new("com.github.nate10j.BetterReactionTime");
    let _ = relm4::set_global_css_from_file(std::path::Path::new("src/styles.css"));
    app.run::<Model>(());
}
