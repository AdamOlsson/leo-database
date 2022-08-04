
pub mod sm {
    use std::{collections::HashMap};

    use crate::{state_machine::session::session::Session};

    pub type Event = str;
    pub type State = str;
    pub type StateMachine<'a> = HashMap<Key<'a>, Value>;
    pub type Function<'a> = fn(&mut Session) -> Result<&'static Event, std::io::Error>;

    #[derive(PartialEq, Eq, Hash)]
    pub struct Key<'a>{
        state: &'a State,
        event: &'a Event
    }

    pub struct Value {
        pub fun: Function<'static>,
        pub state: &'static State,
    }

    pub fn build_key<'a>(state: &'a State, event: &'a Event) -> Key<'a> {
        return Key {state, event}
    }

    pub fn build_value(fun: Function<'static>, state: &'static State) -> Value {
        return Value {fun, state};
    }
    pub fn build_state_machine<'a>() -> StateMachine<'a> {
        return HashMap::new();
    }

    pub fn start<'a>(sm: &'a StateMachine, session: &'a mut Session, event: &Event) {
        next(sm, session, event);
    }

    fn next<'a>(sm: &'a StateMachine, session: &'a mut Session, event: &'a Event) {
        let key: Key = build_key(session.state, event);
        let value: &Value = match sm.get(&key) {
            Some(v) => v,
            None => return, // Is this a bad way to exit recursion?
        };
        let new_event: &Event = (value.fun)(session).unwrap();
        session.state = value.state;

        next(sm, session, new_event);
    }
}