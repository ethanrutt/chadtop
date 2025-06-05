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
