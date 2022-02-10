use std::mem::{forget, replace, zeroed};

pub trait StateMachine
where
    Self: Sized,
{
    type Item;
    fn step(mut self) -> (Self, Option<Self::Item>) {
        let result = self.step_mut();
        (self, result)
    }

    fn step_mut(&mut self) -> Option<Self::Item> {
        unsafe {
            // Replace self with zeroed memory
            let tmp = replace(self, zeroed());
            // Calculate the result using step()
            let (next, result) = tmp.step();
            // Replace self with the result, forget the zeroed memory
            forget(replace(self, next));
            result
        }
    }
}
