extern crate declarative_state_machine;

use declarative_state_machine::state_machine;

state_machine! {
    machine bottle_filler {
        event fill(volume: f32);
        event full();
        event fuel();
        event dump();

        handler fill_bottle(_from: Option<&State>, to: &mut State, volume: f32) {
            if let State::Filling(ref mut filling_volume) = to {
                *filling_volume = volume;
            }
        }

        handler fuel_tank(_from: Option<&State>, to: &mut State) {
            if let State::Idle { ref mut remaining } = to {
                *remaining = 42.0;
            }
        }

        handler bottle_full(from: Option<&State>, to: &mut State) {
            if let Some(State::Filling(ref volume)) = from {
                if let State::Idle { ref mut remaining } = to {
                    *remaining -= volume;
                }
            }
        }

        states {
            Idle {
                remaining: f32,
            },
            Filling(f32),
            Empty,
        }

        transitions {
            // when volume that should be filled by bottle filler is smaller or equal
            // to the remaining capacity of the bottle filler, call the fill_bottle
            // handler.
            Idle { remaining } => Filling : fill(volume) {
                volume <= remaining
            } -> fill_bottle;

            // when volume that should be filled by bottle filler is greater than the
            // remaining capacity of the bottle filler, just go to empty state without
            // calling a handler.
            Idle { remaining } => Empty : fill(volume) {
                volume > remaining
            };

            // when dump event is fired from idle state, go to empty without calling a handler.
            Idle { .. } => Empty : dump();

            // when fuel event is fired from idle state, always call the fuel_tank handler.
            Idle { .. } => Idle : fuel() -> fuel_tank;

            // when full event is fired from filling state, always call bottle_full handler.
            Filling(..) => Idle : full() -> bottle_full;

            // when fuel event is fired from empty state, always call the fuel_tank handler.
            Empty => Idle : fuel() -> fuel_tank;
        }
    }
}

fn main() {
    let _state_machine = bottle_filler::Machine::new();
}
