#![allow(non_snake_case)]
/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 *
 * "Rusty Gui Maker": Main Rusty Gui Maker Class
 *
 * Copyright (c) "2019" Rusty Gui Maker
*/
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder,CommandBuffer, DynamicState};
use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, Subpass, RenderPassAbstract};
use vulkano::image::SwapchainImage;
#[allow(unused)]
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::GraphicsPipelineAbstract;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{AcquireError, PresentMode, SurfaceTransform, Swapchain, SwapchainCreationError};
use vulkano::swapchain;
use vulkano::sync::{GpuFuture, FlushError};
use vulkano::sync;
#[allow(unused)]
use vulkano_win::VkSurfaceBuild;
#[allow(unused)]
use winit::{EventsLoop, Window, WindowBuilder, Event, WindowEvent, Icon, KeyboardInput, ElementState, MouseCursor, MouseButton };

// use std::thread;
// use std::time::Duration;

// IMPORT
//use vulkano_text::{DrawText, DrawTextTrait};
// IMPORT END

//las 2 siguientes y la de Icon para icons
extern crate image;
use std::path::Path;

//min max sizes
use winit::dpi::LogicalSize;
use std::sync::Arc;
// use std::rc::Rc;


// use std::borrow::BorrowMut; //.borrow_mut();

//for the Rusty Gui Maker Structs
pub mod Test;
pub mod Structs;




//ObjectPicker
use vulkano::format::Format;
use vulkano::image::attachment::AttachmentImage;
use vulkano::image::ImageUsage;
use std::iter;




//guero
//Errores Se muere cuando se minimiza
//Genereador de figuras, con mapa de vertex
//cstm callbacks
//kien recive el io, focus, pidiendo el focus o con tab

//ichi
//seleccionar con un id pero no numerico ke sea un nombre,porke ahorita solo hay ids y esos cambian
//enum de colores
//rusty text

//kien sea
//img


//guero not now
//Merge de objetos


// let mut Figures = Structs::Vertex::Figures::addFigure();
//controles 
//cola de mensajes - a los callbacks
//hit test el clikeado, va desde adelante hasta atras para ver si le pico a alguien hasta ke true










//#[derive(Debug, Clone)]
#[derive(Default, Copy, Clone)]
pub struct Vertex {
	position: [f32; 3],
	color: [f32; 3],
}
vulkano::impl_vertex!(Vertex, position, color);

struct ObjectPicker {
	queue: Arc<Queue>,
	dimensions: [u32; 2],

	// Tells the GPU where to write the color
	render_pass: Arc<RenderPassAbstract + Send + Sync>,
	pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,

	// Two attachments -> color and depth
	framebuffer: Arc<FramebufferAbstract + Send + Sync>,

	// color attachment
	image: Arc<AttachmentImage>,

	// Will have the data from `image` copied to it.
	buf: Arc<CpuAccessibleBuffer<[u8]>>,
}

impl ObjectPicker {
	fn new(queue: Arc<Queue>, dimensions: [u32; 2]) -> Self {
		// Create the image to which we are going to render to. This
		// is not a swapchain image as we do not render to screen.
		let image_usage = ImageUsage {
			transfer_source: true, // This is necessary to copy to external buffer
			..ImageUsage::none()
		};

		let image = AttachmentImage::with_usage(
			queue.device().clone(),
			dimensions,
			Format::R8G8B8A8Unorm, // simple format for encoding the ID as a color
			image_usage,
		)
		.unwrap();

		let depth_buffer = AttachmentImage::transient(queue.device().clone(), dimensions, Format::D16Unorm).unwrap();

		let render_pass = Arc::new(
			vulkano::single_pass_renderpass!(
					queue.device().clone(),
					attachments: {
						color: {
							load: Clear,
							store: Store,
							format: Format::R8G8B8A8Unorm,
							samples: 1,
						},
						depth: {
							load: Clear,
							store: DontCare,
							format: Format::D16Unorm,
							samples: 1,
						}
					},
					pass: {
						color: [color],
						depth_stencil: {depth}
					}
			)
			.unwrap(),
		);

		// Use our custom image in the framebuffer.
		let framebuffer = Arc::new(
			Framebuffer::start(render_pass.clone())
				.add(image.clone()).unwrap()
				.add(depth_buffer.clone()).unwrap()
				.build().unwrap(),
		);

		// That is the CPU accessible buffer to which we'll transfer the image content
		// so that we can read the data. It should be as large as 4 the number of pixels (because we
		// store rgba value, so 4 time u8)
		let buf = CpuAccessibleBuffer::from_iter(
			queue.device().clone(),
			BufferUsage::all(),
			(0..dimensions[0] * dimensions[1] * 4).map(|_| 0u8),
		)
		.expect("Failed to create buffer");

		//

		let vs = Structs::Shaders::pick_vs::Shader::load(queue.device().clone()).unwrap();
		let fs = Structs::Shaders::pick_fs::Shader::load(queue.device().clone()).unwrap();

		let pipeline = Arc::new(
			GraphicsPipeline::start()
				//.vertex_input_single_buffer::<Vertex>()
				.vertex_input_single_buffer::<Structs::Vertex::VertexBase>()
				.vertex_shader(vs.main_entry_point(), ())
				.triangle_list()
				.viewports_dynamic_scissors_irrelevant(1)
				.depth_stencil_simple_depth()
				.viewports(iter::once(Viewport {
					origin: [0.0, 0.0],
					dimensions: [dimensions[0] as f32, dimensions[1] as f32],
					depth_range: 0.0..1.0,
				}))
				.fragment_shader(fs.main_entry_point(), ())
				.render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
				.build(queue.device().clone())
				.unwrap(),
		);

		ObjectPicker {
			queue,
			dimensions,
			render_pass,
			pipeline,
			framebuffer,
			image,
			buf,
		}
	}

	fn create_pushconstants(id: usize) -> Structs::Shaders::pick_vs::ty::PushConstants {
		Structs::Shaders::pick_vs::ty::PushConstants {
			color: [
				((id & 0xFF) as f32) / 255.0,
				((id >> 8) & 0xFF) as f32 / 255.0,
				((id >> 16) & 0xFF) as f32 / 255.0,
				1.0,
			], // Transparent means no entity.
		}
	}

	/// Get the ID from a RGBA value. transparent means None
	fn get_entity_id(r: u8, g: u8, b: u8, a: u8) -> Option<usize> {
		if a == 0 {
			None
		} else {
			Some((r as usize) | (g as usize) << 8 | (b as usize) << 16)
		}
	}














	/// Return either ID of picked object or None if did not click on anything
	pub fn pick_objectCanvasFigure( &mut self, x: usize, y: usize, CanvasFigures: &Structs::Vertex::CanvasFigures, ) -> Option<usize> {
		let clear_values = vec![[0.0, 0.0, 0.0, 0.0].into(), 1f32.into()];

		let mut command_buffer_builder = AutoCommandBufferBuilder::primary_one_time_submit(
			self.queue.device().clone(),
			self.queue.family(),
		)
		.unwrap()
		.begin_render_pass(self.framebuffer.clone(), false, clear_values)
		.unwrap();


		for (id, object) in CanvasFigures.VertArray.iter().enumerate() {
			// Now, render all objects and use the ID as push constant.
				let push_constant = ObjectPicker::create_pushconstants(id);
				command_buffer_builder = command_buffer_builder
					.draw(
						self.pipeline.clone(),
						&DynamicState::none(),
						vec![object.clone()],
						(),
						push_constant,
					)
					.unwrap();
		}

		command_buffer_builder = command_buffer_builder.end_render_pass().unwrap();

		// Now copy the image to the CPU accessible buffer.
		command_buffer_builder = command_buffer_builder
			.copy_image_to_buffer(self.image.clone(), self.buf.clone())
			.unwrap();

		let command_buffer = command_buffer_builder.build().unwrap();

		// Execute command buffer and wait for it to finish.
		command_buffer
			.execute(self.queue.clone())
			.unwrap()
			.then_signal_fence_and_flush()
			.unwrap()
			.wait(None)
			.unwrap();

		// ok, at this point the image is in the buffer. We can access it as it is
		// a CPU accessible buffer.
		let buffer_content = self.buf.read().unwrap();

		// Each pixel of the image is represented by a rgba 8-bit value. So to get
		// the index in the vector, we need to multiply by 4.
		let buf_pos = 4 * (y * (self.dimensions[0] as usize) + x);

		let entity_id = ObjectPicker::get_entity_id(
			buffer_content[buf_pos],
			buffer_content[buf_pos + 1],
			buffer_content[buf_pos + 2],
			buffer_content[buf_pos + 3],
		);
		entity_id
	}









	#[allow(unused)]
	/// Return either ID of picked object or None if did not click on anything
	pub fn pick_object( &mut self, x: usize, y: usize, objects: &Vec<Arc<CpuAccessibleBuffer<[Structs::Vertex::VertexBase]>>>, ) -> Option<usize> {
		let clear_values = vec![[0.0, 0.0, 0.0, 0.0].into(), 1f32.into()];

		let mut command_buffer_builder = AutoCommandBufferBuilder::primary_one_time_submit(
			self.queue.device().clone(),
			self.queue.family(),
		)
		.unwrap()
		.begin_render_pass(self.framebuffer.clone(), false, clear_values)
		.unwrap();

		// Now, render all objects and use the ID as push constant.
		for (id, object) in objects.iter().enumerate() {
			let push_constant = ObjectPicker::create_pushconstants(id);
			command_buffer_builder = command_buffer_builder
				.draw(
					self.pipeline.clone(),
					&DynamicState::none(),
					vec![object.clone()],
					(),
					push_constant,
				)
				.unwrap();
		}

		command_buffer_builder = command_buffer_builder.end_render_pass().unwrap();

		// Now copy the image to the CPU accessible buffer.
		command_buffer_builder = command_buffer_builder
			.copy_image_to_buffer(self.image.clone(), self.buf.clone())
			.unwrap();

		let command_buffer = command_buffer_builder.build().unwrap();

		// Execute command buffer and wait for it to finish.
		command_buffer
			.execute(self.queue.clone())
			.unwrap()
			.then_signal_fence_and_flush()
			.unwrap()
			.wait(None)
			.unwrap();

		// ok, at this point the image is in the buffer. We can access it as it is
		// a CPU accessible buffer.
		let buffer_content = self.buf.read().unwrap();

		// Each pixel of the image is represented by a rgba 8-bit value. So to get
		// the index in the vector, we need to multiply by 4.
		let buf_pos = 4 * (y * (self.dimensions[0] as usize) + x);

		let entity_id = ObjectPicker::get_entity_id(
			buffer_content[buf_pos],
			buffer_content[buf_pos + 1],
			buffer_content[buf_pos + 2],
			buffer_content[buf_pos + 3],
		);
		entity_id
	}
}




pub fn StartRustyInstance(mut RGMinst : Structs::RGMinstance)-> Structs::RGMinstance{
	RGMinst = Structs::RGMinstance::initializeWithWindowStruct(RGMinst);
	//let window = vulkanInstance.Requirements.surface.window();

	let physical = PhysicalDevice::enumerate( &RGMinst.Requirements.instance ).next().unwrap();
	println!("Using device: {} (type: {:?})", physical.name(), physical.ty());

	let surface = RGMinst.Requirements.surface.window();

	//Set Window Title
	if RGMinst.Window.WindowTitle.is_some(){
		surface.set_title( &Option::as_ref( &Arc::new( RGMinst.Window.GetWindowTitle()) ).unwrap() );
	}

	//Set Window icon
	if RGMinst.Window.WindowIcon.is_some() {
		surface.set_window_icon(Some(Option::as_ref(&Arc::new(RGMinst.Window.GetWindowIcon())).unwrap().clone() ));
	}

	//Set initial size
	if RGMinst.Window.Width.is_some()  && RGMinst.Window.Height.is_some()  {
		surface.set_inner_size( LogicalSize::new(  RGMinst.Window.GetWidth().unwrap()  , RGMinst.Window.GetHeight().unwrap() ));
	}

	//Set min size
	if RGMinst.Window.MinWidth.is_some()  && RGMinst.Window.MinHeight.is_some()  {
		surface.set_min_dimensions( Some( LogicalSize::new(  RGMinst.Window.GetMinWidth().unwrap()  , RGMinst.Window.GetMinHeight().unwrap() )));
	}

	//Set max size
	if RGMinst.Window.MaxWidth.is_some()  && RGMinst.Window.MaxHeight.is_some()  {
		surface.set_max_dimensions( Some( LogicalSize::new(  RGMinst.Window.GetMaxWidth().unwrap()  , RGMinst.Window.GetMaxHeight().unwrap() )));
	}

	let queue_family = physical.queue_families().find(|&q| {
		// We take the first queue that supports drawing to our window.
		q.supports_graphics() && RGMinst.Requirements.surface.is_supported(q).unwrap_or(false)
	}).unwrap();


	let device_ext = DeviceExtensions { khr_swapchain: true, .. DeviceExtensions::none() };
	let (device, mut queues) = Device::new(physical, physical.supported_features(), &device_ext, [(queue_family, 0.5)].iter().cloned()).unwrap();

	RGMinst.Setdevice( Some(device) );
	RGMinst.Setqueues( Some(queues) );

	//RGMinst.Setsurface(surface);

	return RGMinst;
}

pub fn ADDFigRustyInstance(mut RGMinst : Structs::RGMinstance, Fig: Structs::Vertex::Figures, Multiplier: f32, XMovement: f32, YMovement: f32, Color: String, func: Structs::Callbacks::CallbackEmun, key: String )-> Structs::RGMinstance{

	RGMinst.CanvasFigures = Structs::Vertex::CanvasFigures::addFigure(RGMinst.CanvasFigures, Fig, RGMinst.device.clone().unwrap().clone(), Multiplier, XMovement, YMovement,Color, func, key);

	return RGMinst;
}


pub fn ADDCSTMFigRustyInstance(mut RGMinst : Structs::RGMinstance, lcVertexBase: Vec<Structs::Vertex::Points>, Multiplier: f32, XMovement: f32, YMovement: f32, Color: String, func: Structs::Callbacks::CallbackEmun, key: String )-> Structs::RGMinstance{

	RGMinst.CanvasFigures = Structs::Vertex::CanvasFigures::addCSTMFigure(RGMinst.CanvasFigures, lcVertexBase, RGMinst.device.clone().unwrap().clone(), Multiplier, XMovement, YMovement,Color, func, key);

	return RGMinst;
}




pub fn UseRustyInstance(mut RGMinst : Structs::RGMinstance) {	//amarrado original

	let s = "world!".to_string();

	let callback = || {
		println!("hello {}", s);
	};

	let mut p = Structs::Callbacks::Processor {
		callback: callback,
		callbackQueue: Vec::new( ),
	};


	let queue = RGMinst.queues.unwrap().next().unwrap();
	let physical = PhysicalDevice::enumerate( &RGMinst.Requirements.instance ).next().unwrap();
	let surface = RGMinst.Requirements.surface.window();

	let ((mut swapchain, images), dimensions) = {
		let caps = RGMinst.Requirements.surface.capabilities(physical).unwrap();
		let usage = caps.supported_usage_flags;
		let alpha = caps.supported_composite_alpha.iter().next().unwrap();
		let format = caps.supported_formats[0].0;

		let initial_dimensions = if let Some(dimensions) = surface.get_inner_size() {
			let dimensions: (u32, u32) = dimensions.to_physical(surface.get_hidpi_factor()).into();// convert to physical pixels
			[dimensions.0, dimensions.1]
		} else {
			// The window no longer exists so exit the application.
			return;
		};

		(
			Swapchain::new( RGMinst.device.clone().unwrap().clone(), RGMinst.Requirements.surface.clone(), caps.min_image_count, format,
				initial_dimensions, 1, usage, &queue, SurfaceTransform::Identity, alpha,
				PresentMode::Fifo, true, None, ) .unwrap(),
			initial_dimensions,
		)
	};



	let vs = Structs::Shaders::vs::Shader::load(RGMinst.device.clone().unwrap().clone()).unwrap();
	let fs = Structs::Shaders::fs::Shader::load(RGMinst.device.clone().unwrap().clone()).unwrap();

	let render_pass = Arc::new(vulkano::single_pass_renderpass!(
		RGMinst.device.clone().unwrap().clone(),
		attachments: {
			color: {
				load: Clear,
				store: Store,
				format: swapchain.format(),
				samples: 1,
			},
			depth: {
				load: Clear,
				store: DontCare,
				format: Format::D16Unorm,
				samples: 1,

			}
		},
		pass: {
			color: [color],
			depth_stencil: {depth}
		}
	).unwrap());


	let pipeline = Arc::new(
		GraphicsPipeline::start()
			.vertex_input_single_buffer()
			.vertex_shader(vs.main_entry_point(), ())
			.triangle_list()
			.viewports_dynamic_scissors_irrelevant(1)
			.depth_stencil_simple_depth()
			.fragment_shader(fs.main_entry_point(), ())
			.render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
			.build(RGMinst.device.clone().unwrap().clone())
			.unwrap(),
	);

	// CREATE DRAWTEXT
	// let mut draw_text = DrawText::new(device.clone(), queue.clone(), swapchain.clone(), &images);
	// let (width, _): (u32, u32) = surface.window().get_inner_size().unwrap().into();
	// let mut x = -200.0;
	// CREATE DRAWTEXT END

	let mut dynamic_state = DynamicState { line_width: None, viewports: None, scissors: None };

	//let mut framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);
	let mut framebuffers = window_size_dependent_setup( RGMinst.device.clone().unwrap().clone(), &images, render_pass.clone(), &mut dynamic_state, );
	// Initialization is finally finished!

	let mut recreate_swapchain = false;

	let mut previous_frame_end = Box::new(sync::now(RGMinst.device.clone().unwrap().clone())) as Box<dyn GpuFuture>;



	let mut object_picker = ObjectPicker::new(queue.clone(), dimensions);
	let mut mouse_x = 0.0;
	let mut mouse_y = 0.0;
	let mut selected_entity = None;
	let mut clicked_entity = None;


	let mut done = false;
	let mut ExitRequest = false;

	//variantes del cursor
	let cursors = [MouseCursor::Default, MouseCursor::Crosshair, MouseCursor::Hand, MouseCursor::Arrow, MouseCursor::Move, MouseCursor::Text, MouseCursor::Wait, MouseCursor::Help, MouseCursor::Progress, MouseCursor::NotAllowed, MouseCursor::ContextMenu,/* MouseCursor::NoneCursor, */MouseCursor::Cell, MouseCursor::VerticalText, MouseCursor::Alias, MouseCursor::Copy, MouseCursor::NoDrop, MouseCursor::Grab, MouseCursor::Grabbing, MouseCursor::AllScroll, MouseCursor::ZoomIn, MouseCursor::ZoomOut, MouseCursor::EResize, MouseCursor::NResize, MouseCursor::NeResize, MouseCursor::NwResize, MouseCursor::SResize, MouseCursor::SeResize, MouseCursor::SwResize, MouseCursor::WResize, MouseCursor::EwResize, MouseCursor::NsResize, MouseCursor::NeswResize, MouseCursor::NwseResize, MouseCursor::ColResize, MouseCursor::RowResize];
	let mut cursor_idx = 0;

	loop {
		previous_frame_end.cleanup_finished();

		if recreate_swapchain {
			let dimensions = if let Some(dimensions) = surface.get_inner_size() {
				let dimensions: (u32, u32) = dimensions.to_physical(surface.get_hidpi_factor()).into();
				[dimensions.0, dimensions.1]
			} else {
				return;
			};

			let (new_swapchain, new_images) = match swapchain.recreate_with_dimension(dimensions) {
				Ok(r) => r,
				Err(SwapchainCreationError::UnsupportedDimensions) => continue,
				Err(err) => panic!("{:?}", err)
			};

			swapchain = new_swapchain;
			//framebuffers = window_size_dependent_setup(&new_images, render_pass.clone(), &mut dynamic_state);

			framebuffers = window_size_dependent_setup( RGMinst.device.clone().unwrap().clone(), &new_images, render_pass.clone(), &mut dynamic_state, );

			// RECREATE DRAWTEXT ON RESIZE
			// draw_text = DrawText::new(device.clone(), queue.clone(), swapchain.clone(), &new_images);
			// RECREATE DRAWTEXT ON RESIZE END

			recreate_swapchain = false;
		}


		// SPECIFY TEXT TO DRAW
		// if x > width as f32 {
		//     x = 0.0;
		// }
		// else {
		//     x += 0.4;
		// }

		// draw_text.queue_text(200.0, 50.0, 20.0, [1.0, 1.0, 1.0, 1.0], "hola dariog.");
		// draw_text.queue_text(20.0, 200.0, 190.0, [0.0, 1.0, 1.0, 1.0], "Hello world!");
		// draw_text.queue_text(x, 350.0, 70.0, [0.51, 0.6, 0.74, 1.0], "Ichi: ( ͡° ͜ʖ ͡°)");
		// draw_text.queue_text(50.0, 350.0, 70.0, [1.0, 1.0, 1.0, 1.0], "Overlappp");
		// SPECIFY TEXT TO DRAW END


		let (image_num, acquire_future) = match swapchain::acquire_next_image(swapchain.clone(), None) {
			Ok(r) => r,
			Err(AcquireError::OutOfDate) => {
				recreate_swapchain = true;
				continue;
			},
			Err(err) => panic!("{:?}", err)
		};

		//let clear_values = vec!([1.0, 1.0, 1.0, 1.0].into());

		  // Specify the color to clear the framebuffer with i.e. blue
		let clear_values = vec![[0.0, 0.0, 0.0, 1.0].into(), 1f32.into()];

		let mut command_buffer_builder =
			AutoCommandBufferBuilder::primary_one_time_submit(RGMinst.device.clone().unwrap().clone(), queue.family()).unwrap()
			.begin_render_pass(framebuffers[image_num].clone(), false, clear_values).unwrap();

		/*let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family()).unwrap()
			.begin_render_pass(framebuffers[image_num].clone(), false, clear_values)
			.unwrap()
			.draw(pipeline.clone(), &dynamic_state, vertex_Pixel.clone(), (), ()).unwrap()
			.end_render_pass()
			.unwrap()
			// DRAW THE TEXT
			// .draw_text(&mut draw_text, image_num)
			// DRAW THE TEXT END
			.build().unwrap();
		*/

		for (i, obj) in RGMinst.CanvasFigures.VertArray.iter().enumerate() {
		// for (i, obj) in objects.iter().enumerate() {
			let pc = {

				if let Some(clicked_idx) = clicked_entity {
					if clicked_idx == i {
						println!("Le picaste a {}", clicked_idx);

						if clicked_idx == 0{
							// println!("00000000000000000 {:?}", CanvasFigures.callback[clicked_idx]);
							// let lol =  CanvasFigures.callback[clicked_idx].clone();
							// p.process_events( CanvasFigures.callback[clicked_idx].clone() );
							// let funcionstr =  Structs::Callbacks::CallbackEmun::as_str( &CanvasFigures.callback[clicked_idx].clone());
							p.process_events( Structs::Callbacks::CallbackEmun::ADD );
						}
						else if clicked_idx == 1{
							// println!("1111111111111111 {:?}", CanvasFigures.callback[clicked_idx]);
							p.process_events( Structs::Callbacks::CallbackEmun::DEL );
						}

						clicked_entity = None;
					} else {
					}
				} else{
				}


				if let Some(selected_idx) = selected_entity {
					if selected_idx == i {
						// println!("Le picaste a {}", selected_idx);
						Structs::Shaders::fs::ty::PushConstants { isSelected: 1, Clicked: 1 }
					} else {
						Structs::Shaders::fs::ty::PushConstants { isSelected: 0, Clicked: 0 }
					}
				} else {
					Structs::Shaders::fs::ty::PushConstants { isSelected: 0, Clicked: 0 }
				}
			};
			command_buffer_builder = command_buffer_builder
				.draw(pipeline.clone(), &dynamic_state, obj.clone(), (), pc)
				.unwrap();
		}

		// p.PrintQueue();

		let Queue = p.GetQueue();

		for d in &mut Queue.clone() {
			if let Some(ref mut du) = *d {
				println!(" Realizando: {:?}", du);

				let funcionstr = Structs::Callbacks::CallbackEmun::as_str(&du.clone());

				if funcionstr == "ADD" {
					println!("ADD");

					RGMinst.CanvasFigures = Structs::Vertex::CanvasFigures::addFigure( RGMinst.CanvasFigures, Structs::Vertex::Figures::Plane, RGMinst.device.clone().unwrap().clone(), 0.1, 0.0, 0.5, "CYAN".to_string(), Structs::Callbacks::CallbackEmun::ADD, String::from("Din1"));
					let DinFigureID = Structs::Vertex::CanvasFigures::getFigureID(RGMinst.CanvasFigures.clone() , "Din1".to_string());
					println!("El ID de la figura generada es:{:?}", DinFigureID);
					println!("El Color de la figura generada es:{:?}", Structs::Vertex::CanvasFigures::getFigureColor(RGMinst.CanvasFigures.clone() , DinFigureID));
					println!("El callback de la figura generada es:{:?}", Structs::Vertex::CanvasFigures::getFigureCallback(RGMinst.CanvasFigures.clone() , DinFigureID));
					println!("El callback de la VertexBase generada es:{:?}", Structs::Vertex::CanvasFigures::getFigureVertexBase(RGMinst.CanvasFigures.clone() , DinFigureID));
					println!("El callback de la Multiplier generada es:{:?}", Structs::Vertex::CanvasFigures::getFigureMultiplier(RGMinst.CanvasFigures.clone() , DinFigureID));
					println!("El callback de la XMovement generada es:{:?}", Structs::Vertex::CanvasFigures::getFigureXMovement(RGMinst.CanvasFigures.clone() , DinFigureID));
					println!("El callback de la YMovement generada es:{:?}", Structs::Vertex::CanvasFigures::getFigureYMovement(RGMinst.CanvasFigures.clone() , DinFigureID));

					// CanvasFigures = Structs::Vertex::CanvasFigures::changefigureColor(CanvasFigures.clone(), device.clone(), "BlUE".to_string(), 0 );

					// let mut CanvasFiguresADD = Structs::Vertex::CanvasFigures::createCanvasFigures();
					// CanvasFiguresADD = Structs::Vertex::CanvasFigures::addFigure( CanvasFiguresADD, Structs::Vertex::Figures::Triangle, device.clone(), 0.1, 0.5, 0.5, String::from("DEL"));
					// objects = DinamicAddFigures(&objects.clone(), &CanvasFiguresADD );
				}
				else if funcionstr == "DEL" {
					RGMinst.CanvasFigures = Structs::Vertex::CanvasFigures::delFigure( RGMinst.CanvasFigures, 2);
					//objects = DinamicDelFigure(&objects.clone(), 2 );
				}
				else if funcionstr == "MOD" {
					println!("MOD");
				}
				else if funcionstr == "CSM" {
					println!("CSTM");
				}
				else{
					println!("Funcion no reconocida");
				}
			}
		}
		p.CleanQueue();

		// p.DoQueue();


		// Finish building the command buffer by calling `build`.
		let command_buffer = command_buffer_builder.end_render_pass().unwrap().build().unwrap();


		let future = previous_frame_end.join(acquire_future)
			.then_execute(queue.clone(), command_buffer).unwrap()

			.then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
			.then_signal_fence_and_flush();

		match future {
			Ok(future) => {
				previous_frame_end = Box::new(future) as Box<_>;
			}
			Err(FlushError::OutOfDate) => {
				recreate_swapchain = true;
				previous_frame_end = Box::new(sync::now(RGMinst.device.clone().unwrap().clone())) as Box<_>;
			}
			Err(e) => {
				println!("{:?}", e);
				previous_frame_end = Box::new(sync::now(RGMinst.device.clone().unwrap().clone())) as Box<_>;
			}
		}
		/*
		let mut Event = &Arc::new(&RGMinst.Requirements.events_loop);
		//&Arc::new( RGMinst.Window.GetWindowTitle())
		**Event.poll_events(|event| {
 		*/

		let VulkanoSurfaceClone = RGMinst.Requirements.surface.clone();
		let VulkanoWindowClone = RGMinst.Window.clone();
		let CanvasFigures = RGMinst.CanvasFigures.clone();


		&RGMinst.Requirements.events_loop.poll_events(|event| {
		//events_loop.poll_events(|ev| {
			//println!("Eventos: \"{:?}\"", ev);
			//Motion { axis: 0, value: -4.0 }  (pantalla creo)
			//MouseMotion { delta: (-4.0, 10.0) }
			//CursorMoved { device_id: DeviceId(DeviceId(0)), position: LogicalPosition { x: 461.0, y: 474.0 }, modifiers: ModifiersState { shift: false, ctrl: false, alt: false, logo: false } } 
			//CursorEntered { device_id: DeviceId(DeviceId(0)) }
			//CursorLeft { device_id: DeviceId(DeviceId(0)) }
			//Button { button: 1, state: Released }
			//CloseRequested
			//Key(KeyboardInput { scancode: 2, state: Released, virtual_keycode: Some(Key1), modifiers: ModifiersState { shift: false, ctrl: false, alt: false, logo: false } })
			//KeyboardInput { device_id: DeviceId(DeviceId(0)), input: KeyboardInput { scancode: 2, state: Released, virtual_keycode: Some(Key1), modifiers: ModifiersState { shift: false, ctrl: false, alt: false, logo: false } } }
			//ReceivedCharacter('1')

			//Key(KeyboardInput { scancode: 17, state: Pressed, virtual_keycode: Some(W), modifiers: ModifiersState { shift: false, ctrl: false, alt: false, logo: false } })
			//event: KeyboardInput { device_id: DeviceId(DeviceId(0)), input: KeyboardInput { scancode: 17, state: Pressed, virtual_keycode: Some(W), modifiers: ModifiersState { shift: false, ctrl: false, alt: false, logo: false } } }
			//Setting cursor to "ContextMenu
			//event: ReceivedCharacter('w')

			//Button { button: 2, state: Released } }"
			//MouseInput { device_id: DeviceId(DeviceId(0)), state: Released, button: Middle, modifiers: ModifiersState { shift: false, ctrl: false, alt: false, logo: false } } }"
			//CursorMoved { device_id: DeviceId(DeviceId(0)), position: LogicalPosition { x: 985.0, y: 62.0 }, modifiers: ModifiersState { shift: false, ctrl: false, alt: false, logo: false


			use winit::WindowEvent::*;
			use winit::ElementState::Released;
			use winit::VirtualKeyCode::{N, Y, Escape};
			//println!("Eventos: \"{:?}\"", event);


		match event {
				//Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => done = true,
				Event::WindowEvent { event: WindowEvent::Resized(_), .. } => recreate_swapchain = true,
				//Event::WindowEvent { event: WindowEvent::KeyboardInput , .. } => recreate_swapchain = true,
				Event::WindowEvent { event: WindowEvent::ReceivedCharacter('w') , .. } => {
					println!("le picaste a la W");
					
				},


				// position: LogicalPosition,
				//LogicalPosition { x: 978.0, y: 139.0 }
				//Event::WindowEvent { event: WindowEvent::ReceivedCharacter('w') , .. } => {
				Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
					// println!("CursorMoved");
					// println!("Eventos: \"{:?}\"", event);
					// println!("CursorMoved");

					mouse_x = position.x;
					mouse_y = position.y;
				}//AKI COMO NO HAY COMA SE ENCADENA CON EL SiGUIENTE
				Event::WindowEvent {
					event:
						WindowEvent::MouseInput {
							state: ElementState::Pressed,
							button: MouseButton::Left,
							..
						},
					..
				} => {
					// Do the picking here
					let hidpi_factor = VulkanoSurfaceClone.window().get_hidpi_factor();
					let x = (mouse_x * hidpi_factor).round() as usize;
					let y = (mouse_y * hidpi_factor).round() as usize;
					// selected_entity = object_picker.pick_object(x, y, &objects);
					// clicked_entity  = object_picker.pick_object(x, y, &objects);

					selected_entity = object_picker.pick_objectCanvasFigure(x, y, &CanvasFigures);
					clicked_entity  = object_picker.pick_objectCanvasFigure(x, y, &CanvasFigures);
				},


				/*Event::WindowEvent { event: WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left,  .. }, .. } => {
					println!("le picastel mouse Left");
					println!("Eventos: \"{:?}\"", event);
				},*/
				/*Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput { state: ElementState::Pressed, .. }, .. }, .. } => {
					println!("Setting cursor to \"{:?}\"", cursors[cursor_idx]);
					window.set_cursor(cursors[cursor_idx]);
					if cursor_idx < cursors.len() - 1 {
						cursor_idx += 1;
					} else {
						cursor_idx = 0;
					}
				},
				Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput { scancode: 5, .. }, .. }, .. } => {
					println!("Le picaste \"{:?}\"", 5);
				},*/
				Event::WindowEvent { event: WindowEvent::Moved ( _delta ), ..} => {
					// println!("Se movio la entana \"{:?}\"", 3);
					surface.set_cursor(cursors[cursor_idx]);
					if cursor_idx < cursors.len() - 1 {
						cursor_idx += 1;
					} /*else {
						cursor_idx = 0;
					}*/
				},


				winit::Event::WindowEvent { event, .. } => match event {
					CloseRequested => {
						println!("Are you ready to bid your window farewell? [Y/N]");
						ExitRequest = true;
					}
					KeyboardInput {
						input:
							winit::KeyboardInput {
								virtual_keycode: Some(virtual_code),
								state: Released,
								..
							},
						..
					} => match virtual_code {
						Y => {
							if ExitRequest {
								// This is where you'll want to do any cleanup you need.
								println!("Buh-bye!");
								done = true;
							}
						}
						N => {
							if ExitRequest {
								println!("Your window will continue to stay by your side.");
								done = false;
							}
						}
						winit::VirtualKeyCode::Space => {
							//&RGMinst.Window.SetResizable( Some( !&RGMinst.Window.GetResizable().unwrap()) );
							//VulkanoWindowClone.SetResizable( Some( !&RGMinst.Window.GetResizable().unwrap()) );
							//window.set_resizable( RGMinst.Window.GetResizable().unwrap() );
						}
						Escape => {
							println!("Le picaste Escape");
						}
						_ => (),
					},
					_ => (),
				},
				_ => ()
			}//end match event
		});//end events_loop

		if done { return; }
	}
}



fn window_size_dependent_setup(
	device: Arc<Device>,
	images: &[Arc<SwapchainImage<Window>>],
	render_pass: Arc<RenderPassAbstract + Send + Sync>,
	dynamic_state: &mut DynamicState,
) -> Vec<Arc<FramebufferAbstract + Send + Sync>> {
	let dimensions = images[0].dimensions();

	let viewport = Viewport {
		origin: [0.0, 0.0],
		dimensions: [dimensions[0] as f32, dimensions[1] as f32],
		depth_range: 0.0 .. 1.0,
	};

	dynamic_state.viewports = Some(vec![viewport]);

	images.iter().map(|image| {
			let depth_buffer =
				AttachmentImage::transient(device.clone(), dimensions, Format::D16Unorm).unwrap();
			Arc::new(
				Framebuffer::start(render_pass.clone())
					.add(image.clone()).unwrap()
					.add(depth_buffer.clone()).unwrap()
					.build().unwrap(),
			) as Arc<FramebufferAbstract + Send + Sync>
		})
		.collect::<Vec<_>>()
}

pub fn load_icon(path: &Path) -> Icon {
	let (icon_rgba, icon_width, icon_height) = {
		let image = image::open(path).expect("Failed to open icon path");
		use image::{GenericImageView, Pixel};
		let (width, height) = image.dimensions();
		let mut rgba = Vec::with_capacity((width * height) as usize * 4);
		for (_, _, pixel) in image.pixels() {
			rgba.extend_from_slice(&pixel.to_rgba().data);
		}
		(rgba, width, height)
	};
	Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
