#[macro_use] extern crate state_machine;

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
    //let state_machine = BottleFiller::Machine::new();
}
