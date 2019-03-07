#![feature(proc_macro_hygiene)]

#[macro_use] extern crate state_machine;

state_machine! {
    machine bottle_filler {
        event run(old: &mut State, new: &mut State) {
            println!("Run application");
        }

        event pause(old: &mut State, new: &mut State) {
            //println!("Pause application");
        }

        states {
            Stopped(),
            Running(String),
            Paused(),
        }

        event stop(old: &mut State, new: &mut State) {
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
    let state_machine = bottle_filler::Machine::new();
}
