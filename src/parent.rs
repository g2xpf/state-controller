use crate::state::State;

pub trait Parent<P>: State
where
    P: State,
{
}
