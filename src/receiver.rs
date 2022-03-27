use crate::state::State;

pub trait Receiver<Sender>: State
where
    Sender: State,
{
    type Message: Clone;

    fn receive(&mut self, _message: Self::Message) {}
}
