use std::cell::RefCell;
use std::rc::Rc;

pub fn GenHanConstructor<T: Clone>(thing: T) -> impl FnMut(u32) -> Box<dyn FnMut(Option<T>) -> T> {
    let state = Rc::new(RefCell::new(thing));,
    let mut counter: u32 = 0;

    move |input: u32| {
        if input == 0 {
            // Read-only: returns a clone of the current T (ignores argument)
            let s = state.clone();
            Box::new(move |_: Option<T>| s.borrow().clone())
        } else {
            // Only one mutable controller allowed
            if counter == 0 {
                counter += 1;
                let s = state.clone();
                Box::new(move |thing: Option<T>| {
                    // Update state if we receive itt(val)
                    if let Option::Some(val) = thing {
                        *s.borrow_mut() = val;
                    }
                    s.borrow().clone()
                })
            } else {
                panic!("Returned mutable controller multiple times!");
            }
        }
    }
}