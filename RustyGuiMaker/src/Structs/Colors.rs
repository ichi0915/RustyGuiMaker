
#[allow(unused)]
use vulkano::instance::{Instance, PhysicalDevice};
#[allow(unused)]
use vulkano::device::{Device, DeviceExtensions};
#[allow(unused)]
use vulkano::swapchain::{AcquireError, PresentMode, SurfaceTransform, Swapchain, SwapchainCreationError};
#[allow(unused)]
use vulkano::sync::{GpuFuture, FlushError};
#[allow(unused)]
use winit::{EventsLoop, Window, WindowBuilder, Event, WindowEvent, Icon, KeyboardInput, ElementState, MouseCursor, MouseButton };



pub enum ColorList {
	RED,
	GREEN,
	BlUE,
	YELLOW,
	CYAN,
	PURPLE,
	BLACK,
	WHITE,
	CSTM
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Color {
	colorRGB: [f32; 3],
	colorName: String,
}

impl Color {

	//Function that returns red if the value dosent exist
	pub fn SetColor(mut Name: String) -> [f32; 3] {

		Name = Name.to_uppercase();

		if Name.eq("RED"){
			return [1.0, 0.0, 0.0];
		}
		else if Name.eq("GREEN"){
			return [0.0, 1.0, 0.0];
		}
		else if Name.eq("BlUE"){
			return [0.0, 0.0, 1.0];
		}
		else if Name.eq("YELLOW"){
			return [1.0, 1.0, 0.0];
		}
		else if Name.eq("CYAN"){
			return [0.0, 1.0, 1.0];
		}
		else if Name.eq("PURPLE"){
			return [1.0, 0.0, 1.0];
		}
		else if Name.eq("BLACK"){
			return [0.0, 0.0, 0.0];
		}
		else if Name.eq("WHITE"){
			return [1.0, 1.0, 1.0];
		}
		else if Name.eq("KHAKI"){
			return [0.76, 0.69, 0.57];
		}
		else if Name.eq("LIGHT-PINK"){
			return [1.0, 0.70, 0.73];
		}
		else{
			//white
			return [1.0, 1.0, 1.0];
		}
	}

	pub fn CustomColor(Color: [f32; 3]) -> [f32; 3] {
		// return [1.0, 1.0, 0.0];
		return Color;
	}
}
