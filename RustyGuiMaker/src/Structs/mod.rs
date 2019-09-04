
use vulkano::instance::*;
//use std::sync::Arc;


//#[derive(Copy, Clone)]
pub struct GuiStruct{
	pub instance : Instance,
	pub x: f32,

	pub WIDTH: u32,// = 800;
	pub HEIGHT: u32,
	//const HEIGHT: u32 = 600;
}







extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

use std::sync::Arc;
use std::collections::HashSet;

use winit::{EventsLoop, WindowBuilder, Window, dpi::LogicalSize, Event, WindowEvent};
use vulkano_win::VkSurfaceBuild;

//use vulkano::instance::*;
use vulkano::instance::{
	Instance,
	InstanceExtensions,
	ApplicationInfo,
	Version,
	layers_list,
	PhysicalDevice,
};
use vulkano::instance::debug::{DebugCallback, MessageTypes};
use vulkano::device::{Device, DeviceExtensions, Queue, Features};
use vulkano::swapchain::{
	Surface,
	Capabilities,
	ColorSpace,
	SupportedPresentModes,
	PresentMode,
	Swapchain,
	CompositeAlpha,
	acquire_next_image,
};
use vulkano::format::Format;
use vulkano::image::{ImageUsage, swapchain::SwapchainImage};
use vulkano::sync::{SharingMode, GpuFuture};
use vulkano::pipeline::{
	GraphicsPipeline,
	vertex::BufferlessDefinition,
	vertex::BufferlessVertices,
	viewport::Viewport,
};
use vulkano::framebuffer::{
	RenderPassAbstract,
	Subpass,
	FramebufferAbstract,
	Framebuffer,
};
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::command_buffer::{
	AutoCommandBuffer,
	AutoCommandBufferBuilder,
	DynamicState,
};



use winit::{Icon};

type ConcreteGraphicsPipeline = GraphicsPipeline<BufferlessDefinition, Box<PipelineLayoutAbstract + Send + Sync + 'static>, Arc<RenderPassAbstract + Send + Sync + 'static>>;


#[allow(unused)]
pub struct GuiStructApplication {
	pub instance: Arc<Instance>,
	pub debug_callback: Option<DebugCallback>,

	pub events_loop: EventsLoop,
	pub surface: Arc<Surface<Window>>,

	pub physical_device_index: usize, // can't store PhysicalDevice directly (lifetime issues)
	pub device: Arc<Device>,

	pub graphics_queue: Arc<Queue>,
	pub present_queue: Arc<Queue>,

	pub swap_chain: Arc<Swapchain<Window>>,
	pub swap_chain_images: Vec<Arc<SwapchainImage<Window>>>,

	pub render_pass: Arc<RenderPassAbstract + Send + Sync>,
	// NOTE: We need to the full type of
	// self.graphics_pipeline, because `BufferlessVertices` only
	// works when the concrete type of the graphics pipeline is visible
	// to the command buffer.
	pub graphics_pipeline: Arc<ConcreteGraphicsPipeline>,

	pub swap_chain_framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,

	pub command_buffers: Vec<Arc<AutoCommandBuffer>>,

	pub WIDTHH: u32,
	pub HEIGHTT: u32
	/*pub WIDTHH: u32,

	pub HEIGHTT: u32,*/
}


#[allow(unused)]
pub struct GuiStructApplication2 {

	pub WindowTitle: Option<String>,
	pub WindowIcon: Option<Icon>,

	pub Width: Option<f64>,			//initial Width
	pub Height: Option<f64>,		//initial Height

	pub MinWidth: Option<f64>,
	pub MinHeight: Option<f64>,
	pub MaxWidth: Option<f64>,
	pub MaxHeight: Option<f64>

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