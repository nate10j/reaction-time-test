use std::time::Instant;
use rand::prelude::*;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, SimpleComponent, RelmWidgetExt};

struct Model {
    timer: Option<Instant>,
    reaction_display: ReactionTimeDisplay,
    time: f32,
    waiting_task: Option<tokio::task::JoinHandle<()>>
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
                #[watch]
                set_class_active: ("fail", model.reaction_display == ReactionTimeDisplay::Fail),

                add_css_class: "reaction",

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
                        #[watch]
                        set_label: &format!("{} ms", (model.time * 1000.0).floor()),
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
            waiting_task: None,
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
                        self.waiting_task = Some(tokio::spawn(async move {
                            wait_for_stop(sender).await;
                        }));
                    }
                    ReactionTimeDisplay::Waiting => {
                        if let Some(waiting_task) = &self.waiting_task {
                            waiting_task.abort();
                        }
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
                // if failed, dont react at all.
                if self.reaction_display != ReactionTimeDisplay::Waiting {
                    return;
                }
                self.timer = Some(Instant::now());
                self.reaction_display = ReactionTimeDisplay::Stop;
            }
        }
    }
}

async fn wait_for_stop(sender: ComponentSender<Model>) {
    let wait = {
        let mut rng = rand::rng();
        rng.random_range(500..3000)
    };
    tokio::time::sleep(std::time::Duration::from_millis(wait)).await;
    sender.input(Event::Stop);
}

fn main() {
    let app = RelmApp::new("com.github.nate10j.ReactionTimeTest");
    let _ = relm4::set_global_css_from_file(std::path::Path::new("src/styles.css"));
    app.run::<Model>(());
}
