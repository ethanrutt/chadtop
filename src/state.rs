use crate::proc::Proc;

pub enum CurrentScreen {
    Main,
    Kill,
    KillConfirm,
}

pub struct State {
    pub processes: Vec<Proc>,
    pub proc_index: i32,
    pub current_screen: CurrentScreen,
}

impl State {
    pub fn new() -> State {
        State {
            processes: Vec::<Proc>::new(),
            proc_index: 0,
            current_screen: CurrentScreen::Main,
        }
    }
}
