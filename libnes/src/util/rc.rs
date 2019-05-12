use std::cell::RefCell;
use std::rc::Rc;

pub fn rc_ref<T>(val: T) -> Rc<RefCell<T>> {
    Rc::from(RefCell::from(val))
}
