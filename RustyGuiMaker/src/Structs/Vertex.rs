use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::device::{Device, DeviceExtensions};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, Subpass, RenderPassAbstract};
use vulkano::image::SwapchainImage;
#[allow(unused)]
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{AcquireError, PresentMode, SurfaceTransform, Swapchain, SwapchainCreationError};
use vulkano::swapchain;
use vulkano::sync::{GpuFuture, FlushError};
use vulkano::sync;
#[allow(unused)]
use vulkano_win::VkSurfaceBuild;
#[allow(unused)]
use winit::{EventsLoop, Window, WindowBuilder, Event, WindowEvent, Icon, KeyboardInput, ElementState, MouseCursor, MouseButton };
#[allow(unused)]
use std::thread;
#[allow(unused)]
use std::time::Duration;



use std::sync::Arc;






#[derive(Default, Debug, Clone)]
pub struct VertexBase { position: [f32; 2] }



#[allow(unused)]
pub struct Vertex {
	//pub Vert: VertexBase,
	pub Vert : std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>,

	//pub device: vulkano::device::Device,
	//pub device: std::sync::Arc<vulkano::device::Device>,

	pub Multiplier: f32,
	pub XMovement: f32,
	pub YMovement: f32,
}

#[allow(unused)]
impl Vertex {
	/*pub fn initialize(deviceTmp: Arc<Device> ) -> Self {
	//pub fn initialize(deviceTmp: &'a vulkano::device::Device) -> Self {
	//pub fn initialize(device: Arc<A> ) -> Self {

		let Multiplier = 0.1;
		let XMovement = 0.0;
		let YMovement = 0.0;



		let Vert = {

			vulkano::impl_vertex!(VertexBase, position);

			CpuAccessibleBuffer::from_iter( Arc::new( *deviceTmp.clone() ), BufferUsage::all(), [
				VertexBase { position: [-1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement] },
				VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement] },
				VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement] },

				VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement] },
				VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement] },
				VertexBase { position: [ 1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement] }
			].iter().cloned()).unwrap()
		};

		//let device = deviceTmp.clone();
		//let device = &(*deviceTmp);
		//let device = *Option::as_ref( &Some(*deviceTmp.clone()) ).unwrap();
		//let device = *mut (Light deviceTmp.clone()+'a);


		let mut this = Self {
			Vert,
			Multiplier,
			XMovement,
			YMovement,
			//device,
		};

		this
	}*/
}

/*

#[derive(Default, Debug, Clone)]
struct Vertex {
	position: [f32; 3],
	color: [f32; 3],
}

let vertex_Quad = {
		CpuAccessibleBuffer::from_iter(
			device.clone(),
			BufferUsage::all(),
			[
				Vertex {
					position: [-0.5, -0.5, 0.0],
					color: [1.0, 1.0, 0.0],
				},
				Vertex {
					position: [0.5, -0.5, 0.0],
					color: [1.0, 1.0, 0.0],
				},
				Vertex {
					position: [-0.5, 0.5, 0.0],
					color: [1.0, 1.0, 0.0],
				},
				Vertex {
					position: [0.5, -0.5, 0.0],
					color: [1.0, 1.0, 0.0],
				},
				Vertex {
					position: [0.5, 0.5, 0.0],
					color: [1.0, 1.0, 0.0],
				},
				Vertex {
					position: [-0.5, 0.5, 0.0],
					color: [1.0, 1.0, 0.0],
				},
			]
			.iter()
			.cloned(),
		)
		.unwrap()
	};*/