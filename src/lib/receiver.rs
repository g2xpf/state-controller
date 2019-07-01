use crate::state::State;

pub trait Receiver<Sender>: State
where
    Sender: State,
{
    type Message: Clone;

    fn receive(&mut self, message: Self::Message);
}
