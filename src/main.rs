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
        $old:ident : &State ,
        $new:ident : &State $(,)?
    ) $handler:block ) => {
        impl<'a> Machine<'a> {
            fn $name($old : &State, $new : &State) {
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
        event run(old: &State, new: &State) {
            //println!("Run application");
        }

        event pause(old: &State, new: &State) {
            //println!("Pause application");
        }

        states {
            Stopped(),
            Running(String),
            Paused(),
        }

        event stop(old: &State, new: &State) {
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

fn main() {
    let state_machine = BottleFiller::Machine::new();
}
