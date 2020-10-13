use crate::state::State;

pub trait Receiver<Sender, E = ()>: State<E>
where
    Sender: State<E>,
    E: 'static,
{
    type Message: Clone;

    fn receive(&mut self, _message: Self::Message) {}
}
