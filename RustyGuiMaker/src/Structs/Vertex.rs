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
use std::rc::Rc;




/*

#[derive(Default, Debug, Clone)]
pub struct VertexBase { position: [f32; 2] }



#[allow(unused)]
pub struct Vertex {
	//pub Vert: VertexBase,
	pub Vert : std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>,
//	pub Vert :  Rc<std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>>,
	//pub Vert : Rc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>,

	//pub device: vulkano::device::Device,
	//pub device: std::sync::Arc<vulkano::device::Device>,

	pub Multiplier: f32,
	pub XMovement: f32,
	pub YMovement: f32,
}

#[allow(unused)]
impl Vertex {
	pub fn initialize(deviceTmp: Arc<Device> ) -> Self {
	//pub fn initialize(deviceTmp: Rc<Device> ) -> Self {
	//pub fn initialize(deviceTmp: &'a vulkano::device::Device) -> Self {
	//pub fn initialize(device: Arc<A> ) -> Self {

		let Multiplier = 0.1;
		let XMovement = 0.5;
		let YMovement = 0.0;



		//let Vert = Rc::new({
		let Vert = {

			vulkano::impl_vertex!(VertexBase, position);

			//CpuAccessibleBuffer::from_iter( Arc::new( Rc::try_unwrap(deviceTmp).unwrap() ), BufferUsage::all(), [
			CpuAccessibleBuffer::from_iter(deviceTmp , BufferUsage::all(), [
			//CpuAccessibleBuffer::from_iter( Arc::new( *deviceTmp.clone() ), BufferUsage::all(), [
			//CpuAccessibleBuffer::from_iter( Arc::new( Rc::clone(&deviceTmp) ), BufferUsage::all(), [
			//CpuAccessibleBuffer::from_iter( Rc::new( *deviceTmp.clone() ), BufferUsage::all(), [
				VertexBase { position: [-1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement] },
				VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement] },
				VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement] },

				VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement] },
				VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement] },
				VertexBase { position: [ 1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement] }
			].iter().cloned()).unwrap()
		};
		//}); //Rc

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
	}
}


*/

#[derive(Default, Debug, Clone)]
pub struct VertexBase {
	position: [f32; 3],
	color: [f32; 3],
}

vulkano::impl_vertex!(VertexBase, position, color);

/*
#[allow(unused)]
pub struct vertex_Quad2 {
	pub Vert : std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>,

	pub Multiplier: f32,
	pub XMovement: f32,
	pub YMovement: f32,
}


#[allow(unused)]
impl vertex_Quad2 {
	pub fn initialize(deviceTmp: Arc<Device> ) -> Self {

		let Multiplier = 0.1;
		let XMovement = 0.0;
		let YMovement = 0.0;

		let Vert = {

			CpuAccessibleBuffer::from_iter(deviceTmp , BufferUsage::all(), [
				VertexBase {
					position: [-0.5, -0.5, 0.0],
					color: [1.0, 1.0, 0.0],
				},
				VertexBase {
					position: [0.5, -0.5, 0.0],
					color: [1.0, 1.0, 0.0],
				},
				VertexBase {
					position: [-0.5, 0.5, 0.0],
					color: [1.0, 1.0, 0.0],
				},
				VertexBase {
					position: [0.5, -0.5, 0.0],
					color: [1.0, 1.0, 0.0],
				},
				VertexBase {
					position: [0.5, 0.5, 0.0],
					color: [1.0, 1.0, 0.0],
				},
				VertexBase {
					position: [-0.5, 0.5, 0.0],
					color: [1.0, 1.0, 0.0],
				},
			].iter().cloned()).unwrap()
		};

		let mut this = Self {
			Vert,
			Multiplier,
			XMovement,
			YMovement,
			//device,
		};

		this
	}
}
*/


#[allow(unused)]
pub struct Rectangulo {
	pub Vert : std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>,

	pub Multiplier: f32,
	pub XMovement: f32,
	pub YMovement: f32,
}


#[allow(unused)]
impl Rectangulo {
	pub fn initialize(deviceTmp: Arc<Device>, Multiplier: f32, XMovement: f32, YMovement: f32 ) -> Self {

		let Vert = {
			CpuAccessibleBuffer::from_iter(deviceTmp , BufferUsage::all(), [
				VertexBase { position: [-1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], },
				VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], },
				VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], },

				VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], },
				VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], },
				VertexBase { position: [ 1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], }
			].iter().cloned()).unwrap()
		};

		let mut this = Self {
			Vert,
			Multiplier,
			XMovement,
			YMovement,
			//device,
		};

		this
	}
	pub fn initializeDefault(deviceTmp: Arc<Device> ) -> Self {

		let Multiplier = 0.1;
		let XMovement = 0.0;
		let YMovement = 0.0;

		let Vert = {
			CpuAccessibleBuffer::from_iter(deviceTmp , BufferUsage::all(), [
				VertexBase { position: [-1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], },
				VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], },
				VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], },

				VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], },
				VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], },
				VertexBase { position: [ 1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], }
			].iter().cloned()).unwrap()
		};

		let mut this = Self {
			Vert,
			Multiplier,
			XMovement,
			YMovement,
			//device,
		};

		this
	}
}



#[allow(unused)]
pub struct TrianguloEquilatero {
	pub Vert : std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>,

	pub Multiplier: f32,
	pub XMovement: f32,
	pub YMovement: f32,
}


#[allow(unused)]
impl TrianguloEquilatero {
	pub fn initialize(deviceTmp: Arc<Device>, Multiplier: f32, XMovement: f32, YMovement: f32 ) -> Self {

		let Vert = {
			CpuAccessibleBuffer::from_iter(deviceTmp , BufferUsage::all(), [
				VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: [1.0, 0.0, 0.0], },
				VertexBase { position: [ 1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: [1.0, 0.0, 0.0], },
				VertexBase { position: [ 0.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: [1.0, 0.0, 0.0], }
				].iter().cloned()).unwrap()
		};

		let mut this = Self {
			Vert,
			Multiplier,
			XMovement,
			YMovement,
			//device,
		};

		this
	}

	pub fn initializeDefault(deviceTmp: Arc<Device> ) -> Self {

		let Multiplier = 0.1;
		let XMovement = 0.5;
		let YMovement = 0.0;

		let Vert = {
			CpuAccessibleBuffer::from_iter(deviceTmp , BufferUsage::all(), [
				VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: [1.0, 0.0, 0.0], },
				VertexBase { position: [ 1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: [1.0, 0.0, 0.0], },
				VertexBase { position: [ 0.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: [1.0, 0.0, 0.0], }
				].iter().cloned()).unwrap()
		};

		let mut this = Self {
			Vert,
			Multiplier,
			XMovement,
			YMovement,
			//device,
		};

		this
	}
}


#[allow(unused)]
pub struct Linea {
	//pub Vert : std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>,

	//pub VertArray = TrianguloEquilatero
	pub VertArray: Vec< std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>>,

}


#[allow(unused)]
impl Linea {
	pub fn initialize(deviceTmp: Arc<Device>, Size: i8, Multiplier: f32, XMovement: f32, YMovement: f32  ) -> Self {

	let mut VertArray = Vec::new();

	let Rec = Rectangulo::initialize( deviceTmp.clone(), Multiplier, XMovement, YMovement);

	for i in (0..).take(Size as usize){

		//la siguiente 3 lineas e sun ejemplo por si no me llegaran los valores en un initializeDefault
		let Multiplier = Rec.Multiplier.clone();
		let X = Rec.XMovement.clone();
		let Y = Rec.YMovement.clone();

		let NewRec = Rectangulo::initialize( deviceTmp.clone(), Multiplier, X + Multiplier * i as f32, Y);
		let Vert = NewRec.Vert.clone();

		VertArray.push( Vert );
	}


	let mut this = Self {
			VertArray,
		};

		this
	}

}