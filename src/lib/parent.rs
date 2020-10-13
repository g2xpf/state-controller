use crate::state::State;

pub trait Parent<P, E = ()>: State<E>
where
    P: State<E>,
    E: 'static,
{
}
