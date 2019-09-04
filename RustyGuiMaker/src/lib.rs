#![allow(non_snake_case)]
/**
 ** This Source Code Form is subject to the terms of the Mozilla Public
 ** License, v. 2.0. If a copy of the MPL was not distributed with this
 ** file, You can obtain one at http://mozilla.org/MPL/2.0/.
 **
 **
 ** "Rusty Gui Maker": Main Rusty Gui Maker Class
 **
 ** Copyright (c) "2019" Rusty Gui Maker
 **/

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::device::{Device, DeviceExtensions};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, Subpass, RenderPassAbstract};
use vulkano::image::SwapchainImage;
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{AcquireError, PresentMode, SurfaceTransform, Swapchain, SwapchainCreationError};
use vulkano::swapchain;
use vulkano::sync::{GpuFuture, FlushError};
use vulkano::sync;

use vulkano_win::VkSurfaceBuild;

use winit::{EventsLoop, Window, WindowBuilder, Event, WindowEvent, Icon, KeyboardInput, ElementState, MouseCursor, MouseButton };

use std::thread;
use std::time::Duration;


//las 2 siguientes y la de Icon para icons
extern crate image;
use std::path::Path;

//min max sizes
use winit::dpi::LogicalSize;

use std::sync::Arc;

//para usar mis modulos
pub mod Test;
pub mod Structs;

pub fn CreateDefaultRustyInstance() -> Structs::GuiStructApplication2{

	let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../Resources/RGMLogo.png");

	let vulkanInstance: Structs::GuiStructApplication2 = Structs::GuiStructApplication2 {

		WindowTitle: Some(String::from( "Rusty Gui maker" )),
		WindowIcon:  Some(load_icon(Path::new(path))),

		Width: Some(800.0),
		Height: Some(600.0),

		MinWidth: Some(400.0),
		MinHeight: Some(200.0),
		MaxWidth: Some(1024.0),
		MaxHeight: Some(768.0),
	};

	return vulkanInstance;
}


pub fn UseRustyInstance(WindowStruct : Structs::GuiStructApplication2) {

	let instance = {
		let extensions = vulkano_win::required_extensions();

		Instance::new(None, &extensions, None).unwrap()
	};


	let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
	println!("Using device: {} (type: {:?})", physical.name(), physical.ty());


	let mut events_loop = EventsLoop::new();
	//let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../Resources/RGMLogo.png");
	//let icon = load_icon(Path::new(path));
	//let surface = WindowBuilder::new().with_title("Rusty").with_window_icon(Some(icon)).build_vk_surface(&events_loop, instance.clone()).unwrap();
	let surface = WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();
	let window = surface.window();

	//Set Window Title
	if WindowStruct.WindowTitle.is_some() {
		window.set_title( &WindowStruct.WindowTitle.unwrap() );
	}

	//Set Window icon
	if WindowStruct.WindowIcon.is_some() {
		window.set_window_icon( Some(WindowStruct.WindowIcon.unwrap()) );
	}

	//Set initial size
	if WindowStruct.Width.is_some()  && WindowStruct.Height.is_some()  {
		//window.set_inner_size( LogicalSize::new( 200.0, 500.0 ) );
		window.set_inner_size( LogicalSize::new(WindowStruct.Width.unwrap(), WindowStruct.Height.unwrap()) );
	}

	//Set min size
	if WindowStruct.MinWidth.is_some()  && WindowStruct.MinHeight.is_some()  {
		//window.set_min_dimensions(Some(LogicalSize::new(400.0, 200.0)));
		window.set_min_dimensions( Some( LogicalSize::new(WindowStruct.MinWidth.unwrap(), WindowStruct.MinHeight.unwrap())) );
	}

	//Set max size
	if WindowStruct.MaxWidth.is_some()  && WindowStruct.MaxHeight.is_some()  {
		//window.set_max_dimensions(Some(LogicalSize::new(1024.0, 768.0)));
		window.set_max_dimensions( Some( LogicalSize::new(WindowStruct.MaxWidth.unwrap(), WindowStruct.MaxHeight.unwrap())) );
	}




	let queue_family = physical.queue_families().find(|&q| {
		// We take the first queue that supports drawing to our window.
		q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
	}).unwrap();



	let device_ext = DeviceExtensions { khr_swapchain: true, .. DeviceExtensions::none() };
	let (device, mut queues) = Device::new(physical, physical.supported_features(), &device_ext,
		[(queue_family, 0.5)].iter().cloned()).unwrap();



	let queue = queues.next().unwrap();


	let (mut swapchain, images) = {
		let caps = surface.capabilities(physical).unwrap();

		let usage = caps.supported_usage_flags;

		let alpha = caps.supported_composite_alpha.iter().next().unwrap();

		let format = caps.supported_formats[0].0;

		let initial_dimensions = if let Some(dimensions) = window.get_inner_size() {
			// convert to physical pixels
			let dimensions: (u32, u32) = dimensions.to_physical(window.get_hidpi_factor()).into();
			[dimensions.0, dimensions.1]
		} else {
			// The window no longer exists so exit the application.
			return;
		};

		Swapchain::new(device.clone(), surface.clone(), caps.min_image_count, format,
			initial_dimensions, 1, usage, &queue, SurfaceTransform::Identity, alpha,
			PresentMode::Fifo, true, None).unwrap()
	};


	let mut Multiplier = 0.1;
	let mut XMovement = 0.0;
	let mut YMovement = 0.0;

	//este es un pixel de toda la pantalla, con multiplier lo rebajamos
	let vertex_Pixel = {
		#[derive(Default, Debug, Clone)]
		struct Vertex { position: [f32; 2] }

		//Multiplier = 0.11;

		vulkano::impl_vertex!(Vertex, position);

		CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), [
			Vertex { position: [-1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement] },
			Vertex { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement] },
			Vertex { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement] },

			Vertex { position: [ 1.0 * Multiplier + XMovement, -1.0 * Multiplier + YMovement] },
			Vertex { position: [-1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement] },
			Vertex { position: [ 1.0 * Multiplier + XMovement,  1.0 * Multiplier + YMovement] }
		].iter().cloned()).unwrap()
	};

	//let vertex_Pixel_array: [vertex_Pixel; 5] = [1, 2, 3, 4, 5];
	//let vertex_Pixel_array: [std::sync::Arc<vulkano::buffer::cpu_access::CpuAccessibleBuffer<[main::Vertex]>> ; 5];
	let vertex_Pixel_array: [u32; 5];



	mod vs {
		vulkano_shaders::shader!{
			ty: "vertex",
			src: "
				#version 450

				layout(location = 0) in vec2 position;

				void main() {
					gl_Position = vec4(position, 0.0, 1.0);
				}"
		}
	}

	mod fs {
		vulkano_shaders::shader!{
			ty: "fragment",
			src: "
				#version 450

				layout(location = 0) out vec4 f_color;

				void main() {
					//color triangulo
					f_color = vec4(1.0, 0.0, 0.0, 1.0);
				}
				"
		}
	}

	let vs = vs::Shader::load(device.clone()).unwrap();
	let fs = fs::Shader::load(device.clone()).unwrap();


	let render_pass = Arc::new(vulkano::single_pass_renderpass!(
		device.clone(),
		attachments: {
			color: {
				load: Clear,

				store: Store,

				format: swapchain.format(),
				// TODO:
				samples: 1,
			}
		},
		pass: {
			color: [color],
			depth_stencil: {}
		}
	).unwrap());


	let pipeline = Arc::new(GraphicsPipeline::start()
		.vertex_input_single_buffer()
		.vertex_shader(vs.main_entry_point(), ())
		.triangle_list()
		.viewports_dynamic_scissors_irrelevant(1)
		.fragment_shader(fs.main_entry_point(), ())
		.render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
		.build(device.clone())
		.unwrap());

	let mut dynamic_state = DynamicState { line_width: None, viewports: None, scissors: None };


	let mut framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);

	// Initialization is finally finished!

	let mut recreate_swapchain = false;

	let mut previous_frame_end = Box::new(sync::now(device.clone())) as Box<dyn GpuFuture>;

	let mut done = false;

	//variantes del cursor
	let cursors = [MouseCursor::Default, MouseCursor::Crosshair, MouseCursor::Hand, MouseCursor::Arrow, MouseCursor::Move, MouseCursor::Text, MouseCursor::Wait, MouseCursor::Help, MouseCursor::Progress, MouseCursor::NotAllowed, MouseCursor::ContextMenu,/* MouseCursor::NoneCursor, */MouseCursor::Cell, MouseCursor::VerticalText, MouseCursor::Alias, MouseCursor::Copy, MouseCursor::NoDrop, MouseCursor::Grab, MouseCursor::Grabbing, MouseCursor::AllScroll, MouseCursor::ZoomIn, MouseCursor::ZoomOut, MouseCursor::EResize, MouseCursor::NResize, MouseCursor::NeResize, MouseCursor::NwResize, MouseCursor::SResize, MouseCursor::SeResize, MouseCursor::SwResize, MouseCursor::WResize, MouseCursor::EwResize, MouseCursor::NsResize, MouseCursor::NeswResize, MouseCursor::NwseResize, MouseCursor::ColResize, MouseCursor::RowResize];
	let mut cursor_idx = 0;


	loop {
		previous_frame_end.cleanup_finished();

		if recreate_swapchain {
			let dimensions = if let Some(dimensions) = window.get_inner_size() {
				let dimensions: (u32, u32) = dimensions.to_physical(window.get_hidpi_factor()).into();
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
			framebuffers = window_size_dependent_setup(&new_images, render_pass.clone(), &mut dynamic_state);

			recreate_swapchain = false;
		}

		let (image_num, acquire_future) = match swapchain::acquire_next_image(swapchain.clone(), None) {
			Ok(r) => r,
			Err(AcquireError::OutOfDate) => {
				recreate_swapchain = true;
				continue;
			},
			Err(err) => panic!("{:?}", err)
		};

		let clear_values = vec!([1.0, 1.0, 1.0, 1.0].into());

		let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family()).unwrap()
			.begin_render_pass(framebuffers[image_num].clone(), false, clear_values)
			.unwrap()

			.draw(pipeline.clone(), &dynamic_state, vertex_Pixel.clone(), (), ()).unwrap()

			.draw(pipeline.clone(), &dynamic_state, vertex_Pixel.clone(), (), ()).unwrap()

			.end_render_pass()
			.unwrap()

			.build().unwrap();

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
				previous_frame_end = Box::new(sync::now(device.clone())) as Box<_>;
			}
			Err(e) => {
				println!("{:?}", e);
				previous_frame_end = Box::new(sync::now(device.clone())) as Box<_>;
			}
		}


		//thread::sleep(Duration::from_millis(1000));

		events_loop.poll_events(|ev| {
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

			match ev {
				//Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => done = true,
				Event::WindowEvent { event: WindowEvent::Resized(_), .. } => recreate_swapchain = true,
				//Event::WindowEvent { event: WindowEvent::KeyboardInput , .. } => recreate_swapchain = true,
				Event::WindowEvent { event: WindowEvent::ReceivedCharacter('w') , .. } => {
					println!("le picaste a la W");
				},
				// position: LogicalPosition,
				//LogicalPosition { x: 978.0, y: 139.0 }
				Event::WindowEvent { event: WindowEvent::CursorMoved { .. }, .. } => {
					println!("CursorMoved");
					println!("Eventos: \"{:?}\"", ev);
					println!("CursorMoved");
				},
				Event::WindowEvent { event: WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left,  .. }, .. } => {
					println!("le picastel mouse Left");
					println!("Eventos: \"{:?}\"", ev);
				},
				Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput { state: ElementState::Pressed, .. }, .. }, .. } => {
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
				},
				Event::WindowEvent { event: WindowEvent::Moved ( delta ), ..} => {
					println!("Se movio la entana \"{:?}\"", 3);
					window.set_cursor(cursors[cursor_idx]);
					if cursor_idx < cursors.len() - 1 {
						cursor_idx += 1;
					} /*else {
						cursor_idx = 0;
					}*/
				},
				_ => ()
			}
		});



		events_loop.run_forever(|event| {
			use winit::WindowEvent::*;
			use winit::ElementState::Released;
			use winit::VirtualKeyCode::{N, Y};

			match event {
				winit::Event::WindowEvent { event, .. } => match event {
					CloseRequested => {
						println!("Are you ready to bid your window farewell? [Y/N]");
						done = true;
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
							if done {
								// This is where you'll want to do any cleanup you need.
								println!("Buh-bye!");

								return winit::ControlFlow::Break;
							}
						}
						N => {
							if done {
								println!("Your window will continue to stay by your side.");
								done = false;
							}
						}
						_ => (),
					},
					_ => (),
				},
				_ => (),
			}

			winit::ControlFlow::Continue
		});


		if done { return; }
	}

}



/// This method is called once during initialization, then again whenever the window is resized
pub fn window_size_dependent_setup(
	images: &[Arc<SwapchainImage<Window>>],
	render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
	dynamic_state: &mut DynamicState
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
	let dimensions = images[0].dimensions();

	let viewport = Viewport {
		origin: [0.0, 0.0],
		dimensions: [dimensions[0] as f32, dimensions[1] as f32],
		depth_range: 0.0 .. 1.0,
	};
	dynamic_state.viewports = Some(vec!(viewport));

	images.iter().map(|image| {
		Arc::new(
			Framebuffer::start(render_pass.clone())
				.add(image.clone()).unwrap()
				.build().unwrap()
		) as Arc<dyn FramebufferAbstract + Send + Sync>
	}).collect::<Vec<_>>()
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


/*
pub fn CreateInstance() -> Arc<Instance> {

	let instance = {
		// When we create an instance, we have to pass a list of extensions that we want to enable.
		//
		// All the window-drawing functionalities are part of non-core extensions that we need
		// to enable manually. To do so, we ask the `vulkano_win` crate for the list of extensions
		// required to draw to a window.
		let extensions = vulkano_win::required_extensions();

		// Now creating the instance.
		Instance::new(None, &extensions, None).unwrap()
	};

	return instance;
}*/
/*

pub fn CreatePhysicalDevice<'a>(instance : &Arc<Instance>) -> &'a PhysicalDevice {
	//x: &'a str
	//let physical: &'a PhysicalDevice = &PhysicalDevice::enumerate(&instance).next().unwrap();

	static mut physical: &'static PhysicalDevice = &PhysicalDevice::enumerate(&instance).next().unwrap();
	// Some little debug infos.
	println!("Using device: {} (type: {:?})", physical.name(), physical.ty());

	return physical;
}*/


/*
struct Context<'s>(&'s str);

struct Parser<'c, 's> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
*/