// TODO: Use `Rc` and `RefCell` to implement `DropTracker<T>`, a wrapper around a value of type `T`
//  that increments a shared `usize` counter every time the wrapped value is dropped.

use std::cell::RefCell;
use std::rc::Rc;

pub struct DropTracker<T> {
    value: T,
    counter: Rc<RefCell<usize>>,
}

impl<T> DropTracker<T> {
    pub fn new(value: T, counter: Rc<RefCell<usize>>) -> Self {
        Self { value, counter }
    }
}

impl<T> Drop for DropTracker<T> {
    fn drop(&mut self) {
        *self.counter.borrow_mut() += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let counter = Rc::new(RefCell::new(0));
        let _ = DropTracker::new((), Rc::clone(&counter));
        assert_eq!(*counter.borrow(), 1);
    }

    #[test]
    fn multiple() {
        let counter = Rc::new(RefCell::new(0));

        {
            let a = DropTracker::new(5, Rc::clone(&counter));
            let b = DropTracker::new(6, Rc::clone(&counter));
        }

        assert_eq!(*counter.borrow(), 2);
    }
}
