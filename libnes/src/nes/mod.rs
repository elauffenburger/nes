pub trait Nes {
    fn start() -> ();
    fn reset() -> ();
    fn clock() -> ();
}

pub struct DefaultNes {

}

impl Nes for DefaultNes {
    fn start() {}
    fn reset() {}
    fn clock() {}
}