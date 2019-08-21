
use vulkano::instance::*;
//use std::sync::Arc;

//#[derive(Copy, Clone)]
pub struct GuiStruct{
	pub instance : Instance,
	pub x: f32,
}


/*
impl Copy for GuiStruct {}

impl Clone for GuiStruct {
    fn clone(&self) -> GuiStruct {
        *self
    }
}

#[derive(Debug, Clone, Copy)]
*/