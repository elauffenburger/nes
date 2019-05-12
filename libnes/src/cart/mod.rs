use std::cell::RefCell;
use std::rc::Rc;

use crate::nes::Nes;

pub mod ines;
pub mod mappers;

use ines::iNESLoader;

pub trait CartLoader<TNes>
where
    TNes: Nes,
{
    fn load(&self, nes: Rc<RefCell<TNes>>, cart_data: &[u8]) -> Result<(), String>;
}

pub fn get_cart_loader<TNes>(format: RomFormat) -> Result<impl CartLoader<TNes>, String>
where
    TNes: Nes + 'static,
{
    match format {
        RomFormat::iNes => Ok(iNESLoader::new()),
    }
}

pub enum RomFormat {
    iNes,
}
