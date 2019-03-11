# declarative-state-machine

> This project is WIP and currently in an unstable state

Ever wanted to write state machines in a declarative way with rusts mighty type safety? Well, here it is!

```rust
use declarative_state_machine::state_machine;

state_machine! {
    machine game_lifecycle {
        event run(_old: &mut State, _new: &mut State) {
            println!("Run application");
        }

        event pause(_old: &mut State, _new: &mut State) {
            println!("Pause application");
        }

        states {
            Stopped,
            Paused{ reason: String },
            Running(String),
        }

        event stop(_old: &mut State, _new: &mut State) {
            println!("Stop application");
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
    let _state_machine = game_lifecycle::Machine::new();
}
```

And all this with (quite) meaningful compiler messages:

```
warning: state has no transitions and will never be reached or cause deadlock on construction
  --> src/main.rs:17:13
   |
17 |             Unused,
   |             ^^^^^^
   |
   = help: add a transition to resolve: `Unused => OtherState : some_event`
```

For now all contents of state variants **must** implement the `Default` trait. Maybe this will change in the future.

The initial state of the state machine during construction is the first state in the `states` block.

## TODOs

- [x] Parse syntax of state machine
- [x] Parse semantic of state machine
- [ ] Pass custom data to event handlers (otherwise handlers are quite useless...)
- [ ] Support doc-comments on event handlers and state variants
- [ ] Add serde support
- [ ] Add raft support
- [ ] Add no-std support
- [ ] Write tests
- [ ] Add travis CI configuration
- [ ] Cleanup syntax parsing code by using the `syn` crate
- [ ] Support superstates (nested state machines)

> Want to help? Choose one of the above TODOs and create a pull request with your solution ;-)
