use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
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

// use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
// use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, Subpass, RenderPassAbstract};
// use vulkano::image::SwapchainImage;
// use vulkano::pipeline::GraphicsPipeline;
// use vulkano::pipeline::viewport::Viewport;
// use vulkano::swapchain;
// use vulkano::sync;
// use vulkano_win::VkSurfaceBuild;
// use std::thread;
// use std::time::Duration;



use std::sync::Arc;

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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct VertexBase {
	position: [f32; 3],
	color: [f32; 3],
}

// impl PartialEq for VertexBase {
//     fn eq(&self, other: &Self) -> bool {
//         self.position == other.position
//     }
// }

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


pub enum Figures {
	// Line,
	Plane,
	// Circle,
	Triangle
}
impl Figures {
	pub fn as_str(&self) -> &'static str {
		match *self {
			// Figures::Line => "Line",
			Figures::Plane => "Plane",
			// Figures::Circle => "Circle",
			Figures::Triangle => "Triangle"
		}
	}
	// pub fn as_struct(&self) -> struct {
	// 	match *self {
	// 		Figures::Line => "Line",
	// 		Figures::Plane => "Plane",
	// 		Figures::Circle => "Circle",
	// 		Figures::Triangle => "Triangle"
	// 	}
	// }
}



#[allow(unused)]
#[derive(Debug, Clone)]
pub struct CanvasFigures {
	pub VertArray: Vec< std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>>,
	// pub callback: Vec<Option<String>>
	pub callback: Vec<crate::Structs::Callbacks::CallbackEmun>,
	pub Key: Vec<String>
}


#[allow(unused)]
impl CanvasFigures {

	pub fn createCanvasFigures() -> Self{
		let mut VertArray = Vec::new();
		let mut callback = Vec::new();
		let mut Key = Vec::new();

		let mut this = Self {
			VertArray,
			callback,
			Key,
		};
		this
	}

	pub fn addFigureDefault( CanvasFig: Self, Fig: Figures, deviceTmp: Arc<Device>, key: String ) -> Self{

		let mut KeyExist = false;

		for (id, obj) in CanvasFig.Key.iter().enumerate() {
			if obj.eq(&key) {
				KeyExist = true;
				break;
			}
		}

		let mut VertArray = CanvasFig.VertArray;
		let mut Key = CanvasFig.Key;
		let mut callback = CanvasFig.callback;

		if !KeyExist{

			Key.push( key );
			callback.push( crate::Structs::Callbacks::CallbackEmun::NON );



			// let Vertex = match Fig {
			// 	// Figures::Line => Linea::initialize(deviceTmp: Arc<Device>, Size: i8, Multiplier: f32, XMovement: f32, YMovement: f32),
			// 	Figures::Line => "asd",
			// 	Figures::Plane => Rectangulo::initializeDefault(deviceTmp.clone()),
			// 	Figures::Circle => "Circle",
			// 	Figures::Triangle => TrianguloEquilatero::initializeDefault(deviceTmp.clone())
			// };

			let Figstr = Figures::as_str(&Fig);
			let NewVertex : std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>;

			if(Figstr == "Plane" ){
				NewVertex = Rectangulo::initializeDefault(deviceTmp.clone()).Vert.clone();
			}
			else if(Figstr == "Triangle" ){
				NewVertex = TrianguloEquilatero::initializeDefault(deviceTmp.clone()).Vert.clone();
			}
			else{
				NewVertex = Rectangulo::initializeDefault(deviceTmp.clone()).Vert.clone();
			}

			VertArray.push( NewVertex );
		}


		let mut this = Self {
			VertArray,
			callback,
			Key,
		};
		this
	}


	pub fn addFigure(CanvasFig: Self, Fig: Figures, deviceTmp: Arc<Device>, Multiplier: f32, XMovement: f32, YMovement: f32, func: crate::Structs::Callbacks::CallbackEmun, key: String ) -> Self {


		let mut KeyExist = false;

		for (id, obj) in CanvasFig.Key.iter().enumerate() {
			if obj.eq(&key) {
				KeyExist = true;
				break;
			}
		}

		let mut VertArray = CanvasFig.VertArray;
		let mut Key = CanvasFig.Key;
		let mut callback = CanvasFig.callback;

		if !KeyExist{

			Key.push( key );
			callback.push(  func  );

			let Figstr = Figures::as_str(&Fig);
			let NewVertex : std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>;

			if(Figstr == "Plane" ){
				NewVertex = Rectangulo::initialize( deviceTmp.clone(), Multiplier, XMovement, YMovement).Vert.clone();
			}
			else if(Figstr == "Triangle" ){
				NewVertex = TrianguloEquilatero::initialize( deviceTmp.clone(), Multiplier, XMovement, YMovement).Vert.clone();
			}
			else{
				NewVertex = Rectangulo::initialize( deviceTmp.clone(), Multiplier, XMovement, YMovement).Vert.clone();
			}

			VertArray.push( NewVertex );
		}

		let mut this = Self {
			VertArray,
			callback,
			Key,
		};
		this
	}



	pub fn delFigure( CanvasFig: CanvasFigures, idDel: usize) -> Self {

		//let mut VertArray = Vec::new();
		// let mut VertArray = CanvasFig.VertArray;
		let mut NewVertex = CanvasFigures::createCanvasFigures();

		// let mut VertArray = CanvasFig.VertArray;

		for (id, obj) in CanvasFig.VertArray.iter().enumerate() {
			if idDel != id {
				NewVertex.VertArray.push( obj.clone() );
			}
		}
		for (id, obj) in CanvasFig.callback.iter().enumerate() {
			if idDel != id {
				NewVertex.callback.push( obj.clone() );
			}
		}
		for (id, obj) in CanvasFig.Key.iter().enumerate() {
			if idDel != id {
				NewVertex.Key.push( obj.clone() );
			}
		}
		return NewVertex;
	}


	pub fn GetFigureID( CanvasFig: CanvasFigures, KEY: String) -> usize {

		let mut ID = 9999999999;

		for (id, obj) in CanvasFig.Key.iter().enumerate() {
			//if obj == KEY {
			if obj.eq(&KEY) {
				ID = id;
				break;
			}
		}
		return ID;
	}

	//No se usa
	pub fn GetFigureKEY( CanvasFig: CanvasFigures, idDel: usize) -> String {
		let mut KEY = String::from("");

		for (id, obj) in CanvasFig.VertArray.iter().enumerate() {
			if idDel == id {
				KEY = id.to_string();
				break;
			}
		}
		return KEY;
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