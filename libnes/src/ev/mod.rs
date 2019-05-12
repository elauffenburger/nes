use crate::util::rc_ref;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

pub trait Observable<TEvent> {
    fn subscribe(&mut self, handler: Box<FnMut(&TEvent)>) -> Rc<RefCell<Subject<TEvent>>>;
    fn next(&self, event: TEvent);
}

pub trait Observer<TEvent> {}

pub struct Subject<TEvent> {
    cb: Option<Box<FnMut(&TEvent)>>,
    listeners: Vec<Rc<RefCell<Subject<TEvent>>>>,
    _evtype: Option<PhantomData<TEvent>>,
}

impl<TEvent> Observable<TEvent> for Subject<TEvent> {
    fn subscribe(&mut self, handler: Box<FnMut(&TEvent)>) -> Rc<RefCell<Subject<TEvent>>> {
        let mut sub = Subject::new();
        sub.cb = Some(handler);

        let sub_ref = rc_ref(sub);
        self.listeners.push(sub_ref.clone());

        sub_ref
    }

    fn next(&self, event: TEvent) {
        for listener in &self.listeners {
            let listener = listener.clone();
            let mut listener_ref = listener.borrow_mut();

            if let Some(cb) = &mut listener_ref.cb {
                cb(&event);
            }
        }
    }
}

impl<TEvent> Observer<TEvent> for Subject<TEvent> {}

impl<TEvent> Subject<TEvent> {
    pub fn new() -> Self {
        Subject {
            listeners: vec![],
            cb: None,
            _evtype: None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::{Observable, Subject};

    #[test]
    fn can_subscribe() {
        let mut subject = Subject::new();

        let counter = Rc::from(RefCell::from(Counter { count: 0 }));

        {
            let counter = counter.clone();
            subject.subscribe(Box::from(move |val: &i32| {
                counter.borrow_mut().add(val.clone());
            }));
        }

        subject.next(1);
        subject.next(2);
        subject.next(3);

        assert_eq!(counter.borrow().count, 6);
    }

    #[test]
    fn can_subscribe_mult() {
        let mut subject = Subject::new();

        let counter = Rc::from(RefCell::from(Counter { count: 0 }));

        {
            let counter = counter.clone();
            subject.subscribe(Box::from(move |val: &i32| {
                counter.borrow_mut().add(val.clone());
            }));
        }

        {
            let counter = counter.clone();
            subject.subscribe(Box::from(move |val: &i32| {
                counter.borrow_mut().add(val.clone());
            }));
        }

        subject.next(1);
        subject.next(2);

        assert_eq!(counter.borrow().count, 6);
    }

    struct Counter {
        pub count: i32,
    }

    impl Counter {
        pub fn add(&mut self, val: i32) {
            self.count += val;
        }
    }
}
