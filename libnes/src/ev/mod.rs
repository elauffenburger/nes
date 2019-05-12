use std::marker::PhantomData;
use std::rc::Rc;

pub trait Observable<TEvent> {
    fn subscribe(&mut self, handler: Box<Fn(&TEvent) + 'static>) -> Rc<Subject<TEvent>>;
    fn next(&self, event: TEvent);
}

pub trait Observer<TEvent> {}

pub struct Subject<TEvent> {
    cb: Option<Box<Fn(&TEvent)>>,
    listeners: Vec<Rc<Subject<TEvent>>>,
    _evtype: Option<PhantomData<TEvent>>,
}

impl<TEvent> Observable<TEvent> for Subject<TEvent>
where
    TEvent: 'static,
{
    fn subscribe(&mut self, handler: Box<Fn(&TEvent)>) -> Rc<Subject<TEvent>> {
        let mut sub = Subject::new();
        sub.cb = Some(handler);

        let sub_ref = Rc::from(sub);
        self.listeners.push(sub_ref.clone());

        sub_ref
    }

    fn next(&self, event: TEvent) {
        for listener in &self.listeners {
            if let Some(cb) = &listener.cb {
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
