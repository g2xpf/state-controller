use crate::{
    intermediate_state::IntermediateState,
    shifter_mode::{Pending, Running},
    state::State,
    state_transition::Transition,
    types::{
        state_entry::{IntermediateStateEntry, StateEntry},
        IntermediateStateID, SharedState, StateID,
    },
};

use std::{any::TypeId, cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};
pub struct StateShifter<M, E = ()> {
    pub(crate) states: HashMap<StateID, SharedState<E>>,
    pub(crate) intermediate_states: HashMap<IntermediateStateID, Box<dyn IntermediateState<E>>>,
    pub(crate) next_intermediate_state: Option<IntermediateStateEntry<E>>,
    pub(crate) next_state: Option<StateEntry<E>>,
    shifter_mode: PhantomData<M>,
}

impl<M, E: 'static> StateShifter<M, E> {
    fn insert<S>(&mut self, state: SharedState<E>)
    where
        S: State<E>,
    {
        let state_id = TypeId::of::<S>();
        self.states.insert(state_id, state);
    }

    fn try_insert_transition<F, T, I>(&mut self, mut intermediate_state: I) -> bool
    where
        F: State<E>,
        T: State<E>,
        I: IntermediateState<E>,
    {
        let from = self.get::<F>();
        let to = self.get::<T>();
        if let (Some(from), Some(to)) = (from, to) {
            let transition_location = intermediate_state.transition_location();
            let transition_location = transition_location
                .downcast_mut::<Transition<F, T, E>>()
                .unwrap();
            *transition_location = Transition::<F, T, E>::from_states(from, to);
            let transition_id = (TypeId::of::<F>(), TypeId::of::<T>());
            self.intermediate_states
                .insert(transition_id, Box::new(intermediate_state));
            true
        } else {
            false
        }
    }

    pub(crate) fn insert_intermediate_state_entry(
        &mut self,
        intermediate_state_entry: IntermediateStateEntry<E>,
    ) {
        let IntermediateStateEntry(state_id, state) = intermediate_state_entry;
        // assume that intermediate_state_entry is valid
        self.intermediate_states.insert(state_id, state);
    }

    pub(crate) fn insert_state_entry(&mut self, state_entry: StateEntry<E>) {
        let StateEntry(state_id, state) = state_entry;
        self.states.insert(state_id, state);
    }

    pub(crate) fn get<S>(&self) -> Option<&SharedState<E>>
    where
        S: State<E> + 'static,
    {
        let state_id = TypeId::of::<S>();
        self.states.get(&state_id)
    }

    pub(crate) fn get_cloned<S>(&self) -> Option<SharedState<E>>
    where
        S: State<E> + 'static,
    {
        let state_id = TypeId::of::<S>();
        self.states.get(&state_id).map(Rc::clone)
    }
}

impl<E: 'static> StateShifter<Pending, E> {
    pub fn new<S>(state: Rc<RefCell<S>>) -> Self
    where
        S: State<E> + 'static,
    {
        let mut shifter = StateShifter {
            states: HashMap::new(),
            intermediate_states: HashMap::new(),
            next_state: None,
            next_intermediate_state: None,
            shifter_mode: PhantomData,
        };
        shifter.register_state(state);
        shifter
    }

    pub fn register_state<S>(&mut self, state: Rc<RefCell<S>>)
    where
        S: State<E>,
    {
        self.insert::<S>(state as Rc<RefCell<dyn State<E>>>);
    }

    pub fn try_register_transition<F, T, I>(&mut self, intermediate_state: I) -> bool
    where
        F: State<E>,
        T: State<E>,
        I: IntermediateState<E>,
        E: 'static,
    {
        self.try_insert_transition::<F, T, I>(intermediate_state)
    }

    pub fn run(self) -> StateShifter<Running, E> {
        StateShifter {
            states: self.states,
            intermediate_states: self.intermediate_states,
            next_state: None,
            next_intermediate_state: None,
            shifter_mode: PhantomData,
        }
    }
}

impl<E> StateShifter<Running, E> {
    pub(crate) fn try_take_next(&mut self) -> Option<StateEntry<E>> {
        self.next_state.take()
    }
}
