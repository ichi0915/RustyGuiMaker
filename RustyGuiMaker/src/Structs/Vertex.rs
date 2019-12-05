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


#[derive(Default, Debug, Clone, PartialEq)]
pub struct Points {
	pub Position: [f32; 3],
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct VertexBase {
	position: [f32; 3],
	color: [f32; 3],
	// color: <crate::Structs::Colors::Color>::colorRGB,
	// crate::Structs::Colors::Color::colorRGB
}

vulkano::impl_vertex!(VertexBase, position, color);


#[allow(unused)]
pub struct Rectangulo {
	// pub Vert : std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>,
	pub Vert : Vec<VertexBase>,

	pub Multiplier: f32,
	pub XMovement: f32,
	pub YMovement: f32,
}


#[allow(unused)]
impl Rectangulo {
	pub fn initialize(deviceTmp: Arc<Device>, Multiplier: f32, XMovement: f32, YMovement: f32, Color: String ) -> Self {

		// let Vert = {
		// 	CpuAccessibleBuffer::from_iter(deviceTmp , BufferUsage::all(), [
		// 		// VertexBase { position: [-1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], },
		// 		// VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], },
		// 		// VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], },

		// 		// VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], },
		// 		// VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], },
		// 		// VertexBase { position: [ 1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: [1.0, 1.0, 0.0], }

		// 		VertexBase { position: [-1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), },
		// 		VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), },
		// 		VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), },

		// 		VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), },
		// 		VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), },
		// 		VertexBase { position: [ 1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), }
		// 	].iter().cloned()).unwrap()
		// };

		let mut Vert = Vec::new();

		Vert.push(VertexBase { position: [-1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });
		Vert.push(VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });
		Vert.push(VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });

		Vert.push(VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });
		Vert.push(VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });
		Vert.push(VertexBase { position: [ 1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });

		let mut this = Self {
			Vert,
			Multiplier,
			XMovement,
			YMovement,
		};

		this
	}
	pub fn initializeDefault(deviceTmp: Arc<Device>, Color: String ) -> Self {

		let Multiplier = 0.1;
		let XMovement = 0.0;
		let YMovement = 0.0;

		// let Vert = {
		// 	CpuAccessibleBuffer::from_iter(deviceTmp , BufferUsage::all(), [
		// 		VertexBase { position: [-1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), },
		// 		VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), },
		// 		VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), },

		// 		VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), },
		// 		VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), },
		// 		VertexBase { position: [ 1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), }
		// 	].iter().cloned()).unwrap()
		// };

		let mut Vert = Vec::new();

		Vert.push(VertexBase { position: [-1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });
		Vert.push(VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });
		Vert.push(VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });

		Vert.push(VertexBase { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });
		Vert.push(VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });
		Vert.push(VertexBase { position: [ 1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });


		let mut this = Self {
			Vert,
			Multiplier,
			XMovement,
			YMovement,
		};

		this
	}
}



#[allow(unused)]
pub struct TrianguloEquilatero {
	pub Vert : Vec<VertexBase>,

	pub Multiplier: f32,
	pub XMovement: f32,
	pub YMovement: f32,
}


#[allow(unused)]
impl TrianguloEquilatero {
	pub fn initialize(deviceTmp: Arc<Device>, Multiplier: f32, XMovement: f32, YMovement: f32, Color: String ) -> Self {

		let mut Vert = Vec::new();

		Vert.push( VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });
		Vert.push( VertexBase { position: [ 1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });
		Vert.push( VertexBase { position: [ 0.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });

		let mut this = Self {
			Vert,
			Multiplier,
			XMovement,
			YMovement,
		};

		this
	}

	pub fn initializeDefault(deviceTmp: Arc<Device>, Color: String) -> Self {

		let Multiplier = 0.1;
		let XMovement = 0.5;
		let YMovement = 0.0;

		let mut Vert = Vec::new();

		Vert.push(VertexBase { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });
		Vert.push( VertexBase { position: [ 1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });
		Vert.push( VertexBase { position: [ 0.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement, 0.0], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });


		let mut this = Self {
			Vert,
			Multiplier,
			XMovement,
			YMovement,
		};

		this
	}
}



#[allow(unused)]
pub struct CSTMFIG {
	pub Vert : Vec<VertexBase>,

	pub Multiplier: f32,
	pub XMovement: f32,
	pub YMovement: f32,
}

#[allow(unused)]
impl CSTMFIG {
	pub fn initialize(deviceTmp: Arc<Device>, BasePoints: Vec<Points>, Multiplier: f32, XMovement: f32, YMovement: f32, Color: String ) -> Self {

		let mut Vert = Vec::new();

		for (id, obj) in BasePoints.iter().enumerate() {
			Vert.push(VertexBase { position: [ obj.Position[0] * Multiplier + XMovement, obj.Position[1] * Multiplier + YMovement, obj.Position[2]], color: crate::Structs::Colors::Color::SetColor(Color.clone()), });
		}

		let mut this = Self {
			Vert,
			Multiplier,
			XMovement,
			YMovement,
		};

		this
	}
}




#[derive(Debug, Clone)]
pub enum Figures {
	// Line,
	Plane,
	// Circle,
	Triangle,
	// CSTM
}
impl Figures {
	pub fn as_str(&self) -> &'static str {
		match *self {
			// Figures::Line => "Line",
			Figures::Plane => "Plane",
			// Figures::Circle => "Circle",
			Figures::Triangle => "Triangle",
			// Figures::CSTM => "CSTM"
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
	pub callback: Vec<crate::Structs::Callbacks::CallbackEmun>,
	pub Key: Vec<String>,

	pub VertexBaseCF: Vec<Vec<VertexBase>>,
	pub MultiplierCF: Vec<f32>,
	pub XMovementCF: Vec<f32>,
	pub YMovementCF: Vec<f32>,
	pub ColorCF: Vec<String>,
}


#[allow(unused)]
impl CanvasFigures {

	pub fn createCanvasFigures() -> Self{
		let mut VertArray = Vec::new();
		let mut callback = Vec::new();
		let mut Key = Vec::new();

		let mut VertexBaseCF = Vec::new(); //VertexBaseCF.push(Vec::new());
		let mut MultiplierCF = Vec::new();
		let mut XMovementCF = Vec::new();
		let mut YMovementCF = Vec::new();
		let mut ColorCF = Vec::new();

		let mut this = Self {
			VertArray,
			callback,
			Key,

			VertexBaseCF,
			MultiplierCF,
			XMovementCF,
			YMovementCF,
			ColorCF,
		};
		this
	}

	pub fn GenerateCPU(deviceTmp: Arc<Device>, VertexBase: Vec<VertexBase>) ->  std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>> {

		let Vert = {
			CpuAccessibleBuffer::from_iter(deviceTmp , BufferUsage::all(), VertexBase.iter().cloned()).unwrap()
		};

		return Vert;
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

		let mut VertexBaseCF = CanvasFig.VertexBaseCF;
		let mut MultiplierCF = CanvasFig.MultiplierCF;
		let mut XMovementCF = CanvasFig.XMovementCF;
		let mut YMovementCF = CanvasFig.YMovementCF;
		let mut ColorCF = CanvasFig.ColorCF;

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

			MultiplierCF.push(  0.1  );
			XMovementCF.push(  0.0  );
			YMovementCF.push(  0.0  );

			let Figstr = Figures::as_str(&Fig);
			let NewVertex : std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>;
			let NewVertexBase: Vec<VertexBase>;
			let mut Color = "".to_string();


			if(Figstr == "Plane" ){
				Color = "YELLOW".to_string();
				NewVertexBase =Rectangulo::initializeDefault(deviceTmp.clone(), Color.clone() ).Vert.clone();
				NewVertex = CanvasFigures::GenerateCPU( deviceTmp.clone(), NewVertexBase.clone());
			}
			else if(Figstr == "Triangle" ){
				Color = "RED".to_string();
				NewVertexBase = TrianguloEquilatero::initializeDefault(deviceTmp.clone(), Color.clone() ).Vert.clone();
				NewVertex = CanvasFigures::GenerateCPU( deviceTmp.clone(), NewVertexBase.clone());
			}
			else{
				Color = "Yellow".to_string();
				NewVertexBase = Rectangulo::initializeDefault(deviceTmp.clone(), Color.clone() ).Vert.clone();
				NewVertex = CanvasFigures::GenerateCPU( deviceTmp.clone(), NewVertexBase.clone());
			}

			ColorCF.push( Color );
			VertexBaseCF.push( NewVertexBase );
			VertArray.push( NewVertex );
		}

		let mut this = Self {
			VertArray,
			callback,
			Key,

			VertexBaseCF,
			MultiplierCF,
			XMovementCF,
			YMovementCF,
			ColorCF,
		};
		this
	}


	pub fn addFigure(CanvasFig: Self, Fig: Figures, deviceTmp: Arc<Device>, Multiplier: f32, XMovement: f32, YMovement: f32, Color: String, func: crate::Structs::Callbacks::CallbackEmun, key: String ) -> Self {

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

		let mut VertexBaseCF = CanvasFig.VertexBaseCF;
		let mut MultiplierCF = CanvasFig.MultiplierCF;
		let mut XMovementCF = CanvasFig.XMovementCF;
		let mut YMovementCF = CanvasFig.YMovementCF;
		let mut ColorCF = CanvasFig.ColorCF;

		if !KeyExist{

			Key.push( key );
			callback.push(  func  );

			MultiplierCF.push(  Multiplier  );
			XMovementCF.push(  XMovement  );
			YMovementCF.push(  YMovement  );
			ColorCF.push(  Color.clone()  );

			let Figstr = Figures::as_str(&Fig);
			let NewVertex : std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>;
			let NewVertexBase: Vec<VertexBase>;

			if(Figstr == "Plane" ){
				// NewVertex = Rectangulo::initialize( deviceTmp.clone(), Multiplier, XMovement, YMovement, Color).Vert.clone();
				NewVertexBase = Rectangulo::initialize( deviceTmp.clone(), Multiplier, XMovement, YMovement, Color).Vert.clone();
				NewVertex = CanvasFigures::GenerateCPU( deviceTmp.clone(), NewVertexBase.clone());
			}
			else if(Figstr == "Triangle" ){
				// NewVertex = TrianguloEquilatero::initialize( deviceTmp.clone(), Multiplier, XMovement, YMovement, Color).Vert.clone();
				NewVertexBase = TrianguloEquilatero::initialize( deviceTmp.clone(), Multiplier, XMovement, YMovement, Color).Vert.clone();
				NewVertex = CanvasFigures::GenerateCPU( deviceTmp.clone(), NewVertexBase.clone());
			}
			// else if(Figstr == "CSTM" ){
			// 	NewVertexBase = TrianguloEquilatero::initialize( deviceTmp.clone(), Multiplier, XMovement, YMovement, Color).Vert.clone();
			// 	NewVertex = CanvasFigures::GenerateCPU( deviceTmp.clone(), NewVertexBase.clone());
			// }
			else{
				// NewVertex = Rectangulo::initialize( deviceTmp.clone(), Multiplier, XMovement, YMovement, Color).Vert.clone();
				NewVertexBase = Rectangulo::initialize( deviceTmp.clone(), Multiplier, XMovement, YMovement, Color).Vert.clone();
				NewVertex = CanvasFigures::GenerateCPU( deviceTmp.clone(), NewVertexBase.clone());
			}

			VertexBaseCF.push( NewVertexBase );
			VertArray.push( NewVertex );
		}

		let mut this = Self {
			VertArray,
			callback,
			Key,

			VertexBaseCF,
			MultiplierCF,
			XMovementCF,
			YMovementCF,
			ColorCF,
		};
		this
	}



	pub fn addCSTMFigure(CanvasFig: Self, NewVertexPoints: Vec<Points>, deviceTmp: Arc<Device>, Multiplier: f32, XMovement: f32, YMovement: f32, Color: String, func: crate::Structs::Callbacks::CallbackEmun, key: String ) -> Self {

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

		let mut VertexBaseCF = CanvasFig.VertexBaseCF;
		let mut MultiplierCF = CanvasFig.MultiplierCF;
		let mut XMovementCF = CanvasFig.XMovementCF;
		let mut YMovementCF = CanvasFig.YMovementCF;
		let mut ColorCF = CanvasFig.ColorCF;

		if !KeyExist{

			Key.push( key );
			callback.push(  func  );

			MultiplierCF.push(  Multiplier  );
			XMovementCF.push(  XMovement  );
			YMovementCF.push(  YMovement  );
			ColorCF.push(  Color.clone()  );

			let NewVertex : std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>;

			let NewVertexBase = CSTMFIG::initialize( deviceTmp.clone(), NewVertexPoints, Multiplier, XMovement, YMovement, Color).Vert.clone();
			NewVertex = CanvasFigures::GenerateCPU( deviceTmp.clone(), NewVertexBase.clone());

			VertexBaseCF.push( NewVertexBase );
			VertArray.push( NewVertex );
		}

		let mut this = Self {
			VertArray,
			callback,
			Key,

			VertexBaseCF,
			MultiplierCF,
			XMovementCF,
			YMovementCF,
			ColorCF,
		};
		this
	}


	pub fn addCSTMFigureWithVert(CanvasFig: Self, lcVertexBase: Vec<VertexBase>, deviceTmp: Arc<Device>, Multiplier: f32, XMovement: f32, YMovement: f32, Color: String, func: crate::Structs::Callbacks::CallbackEmun, key: String ) -> Self {

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

		let mut VertexBaseCF = CanvasFig.VertexBaseCF;
		let mut MultiplierCF = CanvasFig.MultiplierCF;
		let mut XMovementCF = CanvasFig.XMovementCF;
		let mut YMovementCF = CanvasFig.YMovementCF;
		let mut ColorCF = CanvasFig.ColorCF;

		if !KeyExist{

			Key.push( key );
			callback.push(  func  );
			MultiplierCF.push(  Multiplier  );
			XMovementCF.push(  XMovement  );
			YMovementCF.push(  YMovement  );
			ColorCF.push(  Color.clone()  );

			let NewVertex : std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>;

			let mut NewVertexPoints: Vec<Points>;
			NewVertexPoints = Vec::new();

			for (id, obj) in lcVertexBase.iter().enumerate() {
				// NewVertexPoints.push( crate::Structs::Vertex::Points { Position: [ 1.0, 1.0, 0.0], });
				NewVertexPoints.push( crate::Structs::Vertex::Points { Position: [ obj.position[0].clone(), obj.position[1].clone(), obj.position[2].clone()], });
			}
			println!("NewVertexPoints: {:?}", NewVertexPoints);

			let NewVertexBase = CSTMFIG::initialize( deviceTmp.clone(), NewVertexPoints, Multiplier, XMovement, YMovement, Color).Vert.clone();
			NewVertex = CanvasFigures::GenerateCPU( deviceTmp.clone(), NewVertexBase.clone());

			VertexBaseCF.push( NewVertexBase );
			VertArray.push( NewVertex );
		}

		let mut this = Self {
			VertArray,
			callback,
			Key,

			VertexBaseCF,
			MultiplierCF,
			XMovementCF,
			YMovementCF,
			ColorCF,
		};
		this
	}



//https://stackoverflow.com/questions/52656814/can-i-alias-fully-qualified-syntax
//https://docs.rs/vulkano/0.13.0/vulkano/buffer/index.html
//https://docs.rs/vulkano/0.13.0/vulkano/buffer/cpu_access/struct.ReadLock.html
//una bonita teoria para changeColorWithID
// for (id, obj) in CanvasFig.VertArray.iter().enumerate() {
// 	if idDel != id {
// 		print!("{:?}", obj);
// 		let asd = CpuAccessibleBuffer::read(obj).unwrap();
// 		// print!("{:?}", cpu_access::ReadLoc::map(asd) );
// 		NewVertex.VertArray.push( obj.clone() );
// 	}
// }


	pub fn changefigureColor( CanvasFig: CanvasFigures, deviceTmp: Arc<Device>, NewColor: String, idDel: usize) -> Self {
		let mut NewCanvasFigures = CanvasFigures::createCanvasFigures();
		//let mut tmpNewCanvasFigures = CanvasFigures::createCanvasFigures();

		//let mut lcVertArray: std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>> = CanvasFig.VertArray[0].clone();
		let mut lccallback: crate::Structs::Callbacks::CallbackEmun = crate::Structs::Callbacks::CallbackEmun::NON;
		let mut lcKey: String = Default::default();
		let mut lcVertexBase: Vec<VertexBase> = Default::default(); //CanvasFig.VertexBaseCF[0].clone();
		let mut lcKMultiplierCF: f32 = Default::default();
		let mut lcXMovementCF: f32 = Default::default();
		let mut lcYMovementCF: f32 = Default::default();

		let mut Exist = false;

		//let tmpNewCanvasFigures = CanvasFigures::addCSTMFigureWithVert( tmpNewCanvasFigures, lcVertexBase, deviceTmp.clone(), lcKMultiplierCF, lcXMovementCF, lcYMovementCF, NewColor, lccallback , lcKey);

		for (id, obj) in CanvasFig.ColorCF.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.ColorCF.push( obj.clone() );
			}
		}
		for (id, obj) in CanvasFig.VertexBaseCF.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.VertexBaseCF.push( obj.clone() );
			}
			else {
				Exist = true;
				lcVertexBase = obj.clone();
				println!("La figura: {:?} es la chida:, {:?}", id, obj.clone());
			}
		}
		for (id, obj) in CanvasFig.VertArray.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.VertArray.push( obj.clone() );
			}
			// else {
			// 	lcVertArray = obj.clone();
			// }
		}
		for (id, obj) in CanvasFig.callback.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.callback.push( obj.clone() );
			}
			else {
				lccallback = obj.clone();
			}
		}
		for (id, obj) in CanvasFig.Key.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.Key.push( obj.clone() );
			}
			else {
				lcKey = obj.clone();
			}
		}
		for (id, obj) in CanvasFig.MultiplierCF.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.MultiplierCF.push( obj.clone() );
			}
			else {
				lcKMultiplierCF = obj.clone();
			}
		}
		for (id, obj) in CanvasFig.XMovementCF.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.XMovementCF.push( obj.clone() );
			}
			else {
				lcXMovementCF = obj.clone();
			}
		}
		for (id, obj) in CanvasFig.YMovementCF.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.YMovementCF.push( obj.clone() );
			}
			else {
				lcYMovementCF = obj.clone();
			}
		}

		if(Exist){
			NewCanvasFigures = CanvasFigures::addCSTMFigureWithVert( NewCanvasFigures, lcVertexBase, deviceTmp.clone(), lcKMultiplierCF, lcXMovementCF, lcYMovementCF, NewColor, lccallback , lcKey);
		}

		return NewCanvasFigures;
	}


	pub fn delFigure( CanvasFig: CanvasFigures, idDel: usize) -> Self {

		//let mut VertArray = Vec::new();
		// let mut VertArray = CanvasFig.VertArray;
		let mut NewCanvasFigures = CanvasFigures::createCanvasFigures();

		// let mut VertArray = CanvasFig.VertArray;

		for (id, obj) in CanvasFig.VertArray.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.VertArray.push( obj.clone() );
			}
		}
		for (id, obj) in CanvasFig.callback.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.callback.push( obj.clone() );
			}
		}
		for (id, obj) in CanvasFig.Key.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.Key.push( obj.clone() );
			}
		}


		for (id, obj) in CanvasFig.VertexBaseCF.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.VertexBaseCF.push( obj.clone() );
			}
		}
		for (id, obj) in CanvasFig.MultiplierCF.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.MultiplierCF.push( obj.clone() );
			}
		}
		for (id, obj) in CanvasFig.XMovementCF.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.XMovementCF.push( obj.clone() );
			}
		}
		for (id, obj) in CanvasFig.YMovementCF.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.YMovementCF.push( obj.clone() );
			}
		}
		for (id, obj) in CanvasFig.ColorCF.iter().enumerate() {
			if idDel != id {
				NewCanvasFigures.ColorCF.push( obj.clone() );
			}
		}

		return NewCanvasFigures;
	}


	pub fn getFigureID( CanvasFig: CanvasFigures, KEY: String) -> usize {
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

	pub fn getFigureColor( CanvasFig: CanvasFigures, idDel: usize) -> String {
		let mut Color = String::from("");

		for (id, obj) in CanvasFig.ColorCF.iter().enumerate() {
			if idDel == id {
				Color = obj.to_string();
				break;
			}
		}
		return Color;
	}

	pub fn getFigureCallback( CanvasFig: CanvasFigures, idDel: usize) -> crate::Structs::Callbacks::CallbackEmun {
		let mut Callback = crate::Structs::Callbacks::CallbackEmun::NON;

		for (id, obj) in CanvasFig.callback.iter().enumerate() {
			if idDel == id {
				Callback = obj.clone();
				break;
			}
		}
		return Callback;
	}

	pub fn getFigureVertexBase( CanvasFig: CanvasFigures, idDel: usize) -> Vec<VertexBase> {
		let mut VertexBase = Vec::new();

		for (id, obj) in CanvasFig.VertexBaseCF.iter().enumerate() {
			if idDel == id {
				VertexBase = obj.clone();
				break;
			}
		}
		return VertexBase;
	}

	pub fn getFigureMultiplier( CanvasFig: CanvasFigures, idDel: usize) -> f32 {
		let mut Multiplier = Default::default();

		for (id, obj) in CanvasFig.MultiplierCF.iter().enumerate() {
			if idDel == id {
				Multiplier = obj.clone();
				break;
			}
		}
		return Multiplier;
	}

	pub fn getFigureXMovement( CanvasFig: CanvasFigures, idDel: usize) -> f32 {
		let mut XMovement = Default::default();

		for (id, obj) in CanvasFig.XMovementCF.iter().enumerate() {
			if idDel == id {
				XMovement = obj.clone();
				break;
			}
		}
		return XMovement;
	}

	pub fn getFigureYMovement( CanvasFig: CanvasFigures, idDel: usize) -> f32 {
		let mut YMovement = Default::default();

		for (id, obj) in CanvasFig.YMovementCF.iter().enumerate() {
			if idDel == id {
				YMovement = obj.clone();
				break;
			}
		}
		return YMovement;
	}


	//No se usa
	pub fn getFigureKEY( CanvasFig: CanvasFigures, idDel: usize) -> String {
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
















// #[allow(unused)]
// pub struct Linea {
// 	//pub Vert : std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>,

// 	//pub VertArray = TrianguloEquilatero
// 	pub VertArray: Vec< std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[VertexBase]>>>,

// }


// #[allow(unused)]
// impl Linea {
// 	pub fn initialize(deviceTmp: Arc<Device>, Size: i8, Multiplier: f32, XMovement: f32, YMovement: f32, Color: String  ) -> Self {

// 	let mut VertArray = Vec::new();

// 	let Rec = Rectangulo::initialize( deviceTmp.clone(), Multiplier, XMovement, YMovement, Color.clone());

// 	for i in (0..).take(Size as usize){

// 		//la siguiente 3 lineas e sun ejemplo por si no me llegaran los valores en un initializeDefault
// 		let Multiplier = Rec.Multiplier.clone();
// 		let X = Rec.XMovement.clone();
// 		let Y = Rec.YMovement.clone();

// 		let NewRec = Rectangulo::initialize( deviceTmp.clone(), Multiplier, X + Multiplier * i as f32, Y, Color.clone());
// 		let Vert = NewRec.Vert.clone();

// 		VertArray.push( Vert );
// 	}


// 	let mut this = Self {
// 			VertArray,
// 		};

// 		this
// 	}

// }