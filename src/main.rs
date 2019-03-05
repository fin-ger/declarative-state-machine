use std::collections::HashMap;

macro_rules! state_machine {
    ( @handler $from:ident => $to:ident : $event:ident ; ) => {
        if event_name == stringify!($event) {
            Self::$event(from, to);
            return true;
        }
    };

    ( @handler $from:ident => $to:ident : $event:ident ; $($tail:tt)+ ) => {
        state_machine!{@handler $from => $to : $event ;}
        state_machine!{@handler $($tail)+}
    };

    ( @destination $from:ident => $to:ident : $event:ident ; ) => {
        if event_name == stringify!($event) {
            return Some(State::$to);
        }
    };

    ( @destination $from:ident => $to:ident : $event:ident ; $($tail:tt)+ ) => {
        state_machine!{@destination $from => $to : $event ;}
        state_machine!{@destination $($tail)+}
    };

    ( @def states { $($states:tt)+ } ) => {
        enum State {
            $($states)*
        }
    };

    ( @def event $name:ident (
        $($param:ident : $t:ty),* $(,)?
    ) $handler:block ) => {
        impl<'a> Machine<'a> {
            fn $name($($param : $t),*) {
                $handler
            }
        }
    };

    ( @def transitions { $($transitions:tt)+ } ) => {
        impl<'a> Machine<'a> {
            fn get_destination(event_name: &str) -> Option<&State> {
                state_machine!{@destination $($transitions)+}
                None
            }

            fn call_handler(&self, event_name: &str, from: &State, to: &State) -> bool {
                state_machine!{@handler $($transitions)+}
                false
            }

            pub fn trigger_event(&mut self, event_name: &str) -> bool {
                let from = self.state;
                let to_option = Self::get_destination(event_name);

                if to_option.is_some() {
                    let to = to_option.unwrap();

                    return self.call_handler(event_name, from, to);
                }

                false
            }
        }
    };

    // parse a definition of a machine
    ( @machine $definition:ident $($n:ident $tts:tt)? { $($content:tt)* } ) => {
        state_machine!{@def $definition $($n $tts)? { $($content)* }}
    };

    // if tail is not empty, issue parsing of the first definition and the tail
    ( @machine $definition:ident $($n:ident $tts:tt)? { $($content:tt)* } $($tail:tt)+ ) => {
        state_machine!{@machine $definition $($n $tts)? { $($content)* }}
        state_machine!{@machine $($tail)+}
    };

    // parse one machine
    ( machine $name:ident { $($machine:tt)+ } ) => {
        mod $name {
            pub struct Machine<'a> {
                state: &'a State,
            }
            state_machine!{@machine $($machine)+}
        }
    };
}

state_machine! {
    machine BottleFiller {
        event run(name: String) {
            //println!("Run application");
        }

        event pause(name: String, duration: i32) {
            //println!("Pause application");
        }

        states {
            Stopped(),
            Running(String),
            Paused(),
        }

        event stop() {
            //println!("Stop application");
        }

        transitions {
            Stopped => Running : run;
            Paused  => Running : run;
            Running => Paused  : pause;
            Running => Stopped : stop;
            Paused  => Stopped : stop;
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum BottleFillerState {
    Waiting,
    Filling { volume: usize },
    Done,
}

impl State for BottleFillerState {}

fn main() {
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
