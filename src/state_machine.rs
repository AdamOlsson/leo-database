use self::state_machine::{Session, Event, State, Key, build_key, Value, build_value, build_state_machine, StateMachine, build_session};

fn start<'a>(session: &Session) -> &'static Event {
    return "";
}

pub fn build_post_video_sm() {
    let state1: &State = "UNITIALIZED";
    let state2: &State = "STARTED";
    let event1: &Event = "START";
    let key1: Key = build_key(state1, event1);
    let value1: Value = build_value(start, state2);

    let mut sm: StateMachine = build_state_machine();
    sm.insert(key1, value1);

    let session: Session = build_session(state1);
}

pub mod state_machine {
    use std::collections::HashMap;

    pub type Event = str;
    pub type State = str;
    pub type StateMachine<'a> = HashMap<Key<'a>, Value<'a>>;
    pub type Function<'a> = fn(&Session) -> &'a Event;

    #[derive(PartialEq, Eq, Hash)]
    pub struct Key<'a>{
        state: &'a State,
        event: &'a Event
    }

    pub struct Value<'a> {
        pub fun: Function<'a>,
        pub state: &'a State,
    }
    pub struct Session<'a>{
        pub state: &'a State,
    }

    pub fn build_key<'a>(state: &'a State, event: &'a Event) -> Key<'a> {
        return Key {state, event}
    }
    
    pub fn build_session<'a>(state: &'a State) -> Session<'a> {
        return Session { state };
    }

    pub fn build_value<'a>(fun: Function<'a>, state: &'a State) -> Value<'a> {
        return Value {fun, state};
    }
    pub fn build_state_machine() -> StateMachine<'static> {
        return HashMap::new();
    }

    pub fn start<'a>(sm: &'static StateMachine, session: &'a mut Session, event: &Event) {
        next(sm, session, event);
    }

    fn next<'a>(sm: &'static StateMachine, session: &'a mut Session, event: &Event) {
        let key: Key = build_key(session.state, event);
        let value: &Value = match sm.get(&key) {
            Some(v) => v,
            None => return, // Is this a bad way to exit recursion?
        };
        let new_event: &Event = (value.fun)(session);
        session.state = value.state;

        next(sm, session, new_event);
    }
}

#[cfg(test)]
mod state_machine_test{
    use super::*;

    fn fun_in_state_1<'a>(session: &Session) -> &'static Event {
        return "goto_state_2";
    }

    fn fun_in_state_2<'a>(session: &Session) -> &'static Event {
        return "goto_state_3";
    }

    #[test]
    fn transition_test() {
        let state_init: &State = "init";
        let state_1: &State = "state_1";
        let goto_state_1: &Event = "goto_state_1";
    
        let mut sm: StateMachine = build_state_machine();
        sm.insert(build_key(state_init, goto_state_1), build_value(fun_in_state_1, state_1));
        
    }
}