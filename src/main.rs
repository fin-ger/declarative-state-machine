#[macro_use] extern crate declarative_state_machine;

state_machine! {
    machine bottle_filler {
        event run(_old: &mut State, _new: &mut State) {
            println!("Run application");
        }

        event pause(_old: &mut State, _new: &mut State) {
            //println!("Pause application");
        }

        states {
            Stopped,
            Paused{ reason: String },
            Running(String),
        }

        event stop(_old: &mut State, _new: &mut State) {
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
    let _state_machine = bottle_filler::Machine::new();
}
