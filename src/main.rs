use std::collections::HashMap;

macro_rules! state_machine {
    ( states { $($name:ident $sig:pat),* $(,)? } ) => {
        "states".to_string()
    };

    ( event $name:ident (
        $($param:ident : $t:ty),* $(,)?
    ) $events:block ) => {
        "event".to_string()
    };

    ( transitions { $( $from:ident => $to:ident : $event:ident ; )* } ) => {
        "transitions".to_string()
    };

    ( ; ) => { };

    ( ; $($tail:tt)+ ) => { state_machine!($($tail)+) };

    ( $i:ident $($n:ident $tts:tt)? { $($bodies:tt)* } $($tail:tt)+ ) => {{
        let mut s = state_machine!($i $($n $tts)? { $($bodies)* });
        s.push_str(&"\n".to_string());
        s.push_str(&state_machine!($($tail)+));
        s
    }};
}

#[derive(PartialEq, Clone, Debug)]
enum BottleFillerState {
    Waiting,
    Filling { volume: usize },
    Done,
}

impl State for BottleFillerState {}

fn main() {
    let sm = state_machine! {
        event run(name: String) {
            println!("Run application");
        };

        event pause(name: String, duration: i32) {
            println!("Pause application");
        };

        states {
            Stopped(),
            Running(String),
            Paused(),
        };

        event stop() {
            println!("Stop application");
        }

        transitions {
            Stopped => Running : run;
            Paused  => Running : run;
            Running => Paused  : pause;
            Running => Stopped : stop;
            Paused  => Stopped : stop;
        }
    };

    println!("{}", sm);

    let mut transitions = HashMap::new();
    transitions.insert(
        "start filling",
        StateMachineTransition::new(
            |old_state| {
                match old_state {
                    BottleFillerState::Waiting => {
                        println!("start filling");

                        Some(BottleFillerState::Filling { volume: 250 })
                    },
                    _ => None,
                }
            },
        ),
    );
    transitions.insert(
        "stop filling",
        StateMachineTransition::new(
            |old_state| {
                match old_state {
                    BottleFillerState::Filling { volume: _ } => {
                        println!("stop filling");

                        Some(BottleFillerState::Done)
                    },
                    _ => None,
                }
            },
        ),
    );
    transitions.insert(
        "bottle removed",
        StateMachineTransition::new(
            |old_state| {
                match old_state {
                    BottleFillerState::Done => {
                        println!("bottle removed");

                        Some(BottleFillerState::Waiting)
                    },
                    _ => None,
                }
            },
        ),
    );

    let mut sm2 = StateMachine::new(BottleFillerState::Waiting, transitions);
    assert_eq!(sm2.trigger_event("start filling"), true);
    assert_eq!(sm2.trigger_event("bottle removed"), false);
    assert_eq!(sm2.trigger_event("stop filling"), true);
    assert_eq!(sm2.trigger_event("bottle removed"), true);
}

trait State: std::cmp::PartialEq + std::clone::Clone {}

struct StateMachineTransition<'a, T> {
    transition_handler: Box<Fn(&T) -> Option<T> + 'a>,
}

impl<'a, T: State> StateMachineTransition<'a, T> {
    fn new<F: Fn(&T) -> Option<T> + 'a>(transition_handler: F) -> Self {
        StateMachineTransition {
            transition_handler: Box::new(transition_handler),
        }
    }
}

struct StateMachine<'a, T> {
    state: T,
    transitions: HashMap<&'a str, StateMachineTransition<'a, T>>,
}

impl<'a, T: State> StateMachine<'a, T> {
    pub fn new(initial_state: T, transitions: HashMap<&'a str, StateMachineTransition<'a, T>>) -> Self {
        StateMachine {
            state: initial_state,
            transitions: transitions,
        }
    }

    pub fn trigger_event(&mut self, event: &str) -> bool {
        let transition_option = self.transitions.get(event);

        if transition_option.is_some() {
            let transition = transition_option.unwrap();
            let new_state_option = (transition.transition_handler)(&self.state);
            if new_state_option.is_some() {
                self.state = new_state_option.unwrap();

                return true;
            }
        }

        return false;
    }
}
