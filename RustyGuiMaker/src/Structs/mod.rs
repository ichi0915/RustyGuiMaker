#[allow(unused)]
use vulkano::instance::*;
//use std::sync::Arc;

extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

#[allow(unused)]
use std::sync::Arc;
#[allow(unused)]
use std::collections::HashSet;
#[allow(unused)]
use winit::{EventsLoop, WindowBuilder, Window, dpi::LogicalSize, Event, WindowEvent};
use vulkano_win::VkSurfaceBuild;
#[allow(unused)]
use vulkano::instance::{
	Instance,
	InstanceExtensions,
	ApplicationInfo,
	Version,
	layers_list,
	PhysicalDevice,
};
#[allow(unused)]
use vulkano::instance::debug::{DebugCallback, MessageTypes};
#[allow(unused)]
use vulkano::device::{Device, DeviceExtensions, Queue, Features};
#[allow(unused)]
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
#[allow(unused)]
use vulkano::format::Format;
#[allow(unused)]
use vulkano::image::{ImageUsage, swapchain::SwapchainImage};
#[allow(unused)]
use vulkano::sync::{SharingMode, GpuFuture};
#[allow(unused)]
use vulkano::pipeline::{
	GraphicsPipeline,
	vertex::BufferlessDefinition,
	vertex::BufferlessVertices,
	viewport::Viewport,
};
#[allow(unused)]
use vulkano::framebuffer::{
	RenderPassAbstract,
	Subpass,
	FramebufferAbstract,
	Framebuffer,
};
#[allow(unused)]
use vulkano::descriptor::PipelineLayoutAbstract;
#[allow(unused)]
use vulkano::command_buffer::{
	AutoCommandBuffer,
	AutoCommandBufferBuilder,
	DynamicState,
};

use winit::{Icon};
use time::{Timespec, Tm};

pub mod Vertex;
pub mod Shaders;
pub mod Callbacks;



#[allow(unused)]
#[derive(Default, Clone)]
pub struct RGMWindow {

	pub WindowTitle: Option<String>,
	pub WindowIcon: Option<Icon>,

	pub Width: Option<f64>,			//initial Width
	pub Height: Option<f64>,		//initial Height

	pub Resizable: Option<bool>,
	pub MinWidth: Option<f64>,
	pub MinHeight: Option<f64>,
	pub MaxWidth: Option<f64>,
	pub MaxHeight: Option<f64>,

	created_at: i64,
	completed_at: Option<i64>
}

#[allow(unused)]
impl RGMWindow {
	//accesores

	pub fn GetWindowTitle(&self) -> &Option<String> {
		return &self.WindowTitle;
	}
	pub fn GetWindowIcon(&self) -> &Option<Icon> {
		return &self.WindowIcon;
	}
	pub fn GetResizable(&self) -> Option<bool> {
		return self.Resizable;
	}
	pub fn GetWidth(&self) -> Option<f64> {
		return self.Width;
	}
	pub fn GetHeight(&self) -> Option<f64> {
		return self.Height;
	}
	pub fn GetMinWidth(&self) -> Option<f64> {
		return self.MinWidth;
	}
	pub fn GetMinHeight(&self) -> Option<f64> {
		return self.MinHeight;
	}
	pub fn GetMaxWidth(&self) -> Option<f64> {
		return self.MaxWidth;
	}
	pub fn GetMaxHeight(&self) -> Option<f64> {
		return self.MaxHeight;
	}


	// Mutable access.
	pub fn SetWindowTitle(&mut self, WindowTitle: Option<String>) {
		self.WindowTitle = WindowTitle;
	}
	pub fn SetWindowIcon(&mut self, WindowIcon: Option<Icon>) {
		self.WindowIcon = WindowIcon;
	}
	pub fn SetResizable(&mut self, Resizable: Option<bool>) {
		self.Resizable = Resizable;
	}
	pub fn SetWidth(&mut self, Width: Option<f64>) {
		self.Width = Width;
	}
	pub fn SetHeight(&mut self, Height: Option<f64>) {
		self.Height = Height;
	}
	pub fn SetMinWidth(&mut self, MinWidth: Option<f64>) {
		self.MinWidth = MinWidth;
	}
	pub fn SetMinHeight(&mut self, MinHeight: Option<f64>) {
		self.MinHeight = MinHeight;
	}
	pub fn SetMaxWidth(&mut self, MaxWidth: Option<f64>) {
		self.MaxWidth = MaxWidth;
	}
	pub fn SetMaxHeight(&mut self, MaxHeight: Option<f64>) {
		self.MaxHeight = MaxHeight;
	}

	pub fn created_at(&self) -> Tm {
		time::at(Timespec::new(self.created_at, 0))
	}
	fn completed_at(&self) -> Option<Tm> {
		self.completed_at.map(|t| time::at(Timespec::new(t,0)))
	}
	pub fn is_done(&self) -> bool {
		self.completed_at.is_some()
	}
}


#[allow(unused)]
//#[derive(Clone)]
pub struct RGMRequirements {
	pub instance: Arc<Instance>,
	pub events_loop: EventsLoop,
	pub surface: Arc<Surface<Window>>,

	created_at: i64,
	completed_at: Option<i64>
}

#[allow(unused)]
impl RGMRequirements {

	pub fn initialize() -> Self {

		let instance = Self::CreateInstance();
		let events_loop = Self::CreateEventLoop();
		let surface = Self::CreateSurface(&events_loop, &instance);

		let created_at = time::now_utc().to_timespec().sec;
		let completed_at = None;

		let mut this = Self {
			instance,
			events_loop,

			surface,
			created_at,
			completed_at,
		};

		this
	}

	pub fn CreateInstance() -> Arc<Instance> {
		let instance = {
			let extensions = vulkano_win::required_extensions();
			Instance::new(None, &extensions, None).unwrap()
		};
		return instance;
	}
	pub fn CreateSurface(events_loop: &EventsLoop, instance: &Arc<Instance>) -> Arc<Surface<Window>>{
		return WindowBuilder::new().build_vk_surface(events_loop, instance.clone()).unwrap();
	}
	pub fn CreateEventLoop() -> EventsLoop{
		return  EventsLoop::new();
	}

	pub fn created_at(&self) -> Tm {
		time::at(Timespec::new(self.created_at, 0))
	}
	fn completed_at(&self) -> Option<Tm> {
		self.completed_at.map(|t| time::at(Timespec::new(t,0)))
	}
	pub fn is_done(&self) -> bool {
		self.completed_at.is_some()
	}
}


#[allow(unused)]
//#[derive(Clone)]
pub struct RGMinstance {
	pub Window: RGMWindow,
	pub Requirements: RGMRequirements
}

#[allow(unused)]
impl RGMinstance {
	pub fn initialize() -> Self {

		let Window = RGMWindow::default();
		let Requirements = RGMRequirements::initialize();

		let mut this = Self {
			Window,
			Requirements,
		};

		this
	}

	pub fn initializeWithWindowStruct(WindowA: RGMWindow) -> Self {

		let Window = WindowA;
		let Requirements = RGMRequirements::initialize();

		let mut this = Self {
			Window,
			Requirements,
		};

		this
	}
}
