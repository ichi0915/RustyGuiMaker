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

// IMPORT
//use vulkano_text::{DrawText, DrawTextTrait};
// IMPORT END

//las 2 siguientes y la de Icon para icons
extern crate image;
use std::path::Path;

//min max sizes
use winit::dpi::LogicalSize;
use std::sync::Arc;

//for the Rusty Gui Maker Structs
pub mod Test;
pub mod Structs;

pub fn UseRustyInstance(WindowStruct : Structs::RGMWindow) {

	let mut vulkanInstance = Structs::RGMinstance::initializeWithWindowStruct(WindowStruct);
	let window = &vulkanInstance.Requirements.surface.window();

	let physical = PhysicalDevice::enumerate( &vulkanInstance.Requirements.instance ).next().unwrap();
	println!("Using device: {} (type: {:?})", physical.name(), physical.ty());


	//Set Window Title
	if vulkanInstance.Window.WindowTitle.is_some(){
		window.set_title( &Option::as_ref( &Arc::new( vulkanInstance.Window.GetWindowTitle()) ).unwrap() );
	}

	//Set Window icon
	if vulkanInstance.Window.WindowIcon.is_some() {
		window.set_window_icon(Some(Option::as_ref(&Arc::new(vulkanInstance.Window.GetWindowIcon())).unwrap().clone() ));
	}

	//Set initial size
	if vulkanInstance.Window.Width.is_some()  && vulkanInstance.Window.Height.is_some()  {
		window.set_inner_size( LogicalSize::new(  vulkanInstance.Window.GetWidth().unwrap()  , vulkanInstance.Window.GetHeight().unwrap() ));
	}

	//Set min size
	if vulkanInstance.Window.MinWidth.is_some()  && vulkanInstance.Window.MinHeight.is_some()  {
		window.set_min_dimensions( Some( LogicalSize::new(  vulkanInstance.Window.GetMinWidth().unwrap()  , vulkanInstance.Window.GetMinHeight().unwrap() )));
	}

	//Set max size
	if vulkanInstance.Window.MaxWidth.is_some()  && vulkanInstance.Window.MaxHeight.is_some()  {
		window.set_max_dimensions( Some( LogicalSize::new(  vulkanInstance.Window.GetMaxWidth().unwrap()  , vulkanInstance.Window.GetMaxHeight().unwrap() )));
	}

	let queue_family = physical.queue_families().find(|&q| {
		// We take the first queue that supports drawing to our window.
		q.supports_graphics() && vulkanInstance.Requirements.surface.is_supported(q).unwrap_or(false)
	}).unwrap();


	let device_ext = DeviceExtensions { khr_swapchain: true, .. DeviceExtensions::none() };
	let (device, mut queues) = Device::new(physical, physical.supported_features(), &device_ext, [(queue_family, 0.5)].iter().cloned()).unwrap();

	let queue = queues.next().unwrap();

	let (mut swapchain, images) = {
		let caps = &vulkanInstance.Requirements.surface.capabilities(physical).unwrap();

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

		Swapchain::new(device.clone(), vulkanInstance.Requirements.surface.clone(), caps.min_image_count, format,
			initial_dimensions, 1, usage, &queue, SurfaceTransform::Identity, alpha,
			PresentMode::Fifo, true, None).unwrap()
	};

	let Multiplier = 0.1;
	let XMovement = 0.0;
	let YMovement = 0.0;

	//este es un pixel de toda la pantalla, con multiplier lo rebajamos
	let vertex_Pixel = {
		#[derive(Default, Debug, Clone)]
		struct Vertex { position: [f32; 2] }

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

	let vs = Structs::vs::Shader::load(device.clone()).unwrap();
	let fs = Structs::fs::Shader::load(device.clone()).unwrap();

	let render_pass = Arc::new(vulkano::single_pass_renderpass!(
		device.clone(),
		attachments: {
			color: {
				load: Clear,
				store: Store,
				format: swapchain.format(),
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

	// CREATE DRAWTEXT
	// let mut draw_text = DrawText::new(device.clone(), queue.clone(), swapchain.clone(), &images);
	// let (width, _): (u32, u32) = surface.window().get_inner_size().unwrap().into();
	// let mut x = -200.0;
	// CREATE DRAWTEXT END

	let mut dynamic_state = DynamicState { line_width: None, viewports: None, scissors: None };

	let mut framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);
	// Initialization is finally finished!

	let mut recreate_swapchain = false;

	let mut previous_frame_end = Box::new(sync::now(device.clone())) as Box<dyn GpuFuture>;

	let mut done = false;
	let mut ExitRequest = false;

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

		let clear_values = vec!([1.0, 1.0, 1.0, 1.0].into());

		let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family()).unwrap()
			.begin_render_pass(framebuffers[image_num].clone(), false, clear_values)
			.unwrap()

			//.draw(pipeline.clone(), &dynamic_state, vertex_Pixel.clone(), (), ()).unwrap()

			.draw(pipeline.clone(), &dynamic_state, vertex_Pixel.clone(), (), ()).unwrap()

			.end_render_pass()
			.unwrap()

			// DRAW THE TEXT
			// .draw_text(&mut draw_text, image_num)
			// DRAW THE TEXT END

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


		&vulkanInstance.Requirements.events_loop.poll_events(|event| {
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
				Event::WindowEvent { event: WindowEvent::CursorMoved { .. }, .. } => {
					println!("CursorMoved");
					println!("Eventos: \"{:?}\"", event);
					println!("CursorMoved");
				},
				Event::WindowEvent { event: WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left,  .. }, .. } => {
					println!("le picastel mouse Left");
					println!("Eventos: \"{:?}\"", event);
				},
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
					println!("Se movio la entana \"{:?}\"", 3);
					window.set_cursor(cursors[cursor_idx]);
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
							//&vulkanInstance.Window.SetResizable( Some( !&vulkanInstance.Window.GetResizable().unwrap()) );
							//window.set_resizable( vulkanInstance.Window.GetResizable().unwrap() );
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