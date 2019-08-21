#![allow(non_snake_case)]

/**
 ** This Source Code Form is subject to the terms of the Mozilla Public
 ** License, v. 2.0. If a copy of the MPL was not distributed with this
 ** file, You can obtain one at http://mozilla.org/MPL/2.0/.
 **
 **
 ** "Test": This module is for all the testing functions.
 **
 ** Copyright (c) "2019" Rusty Gui Maker
 **/

//para vulkano y rust
use vulkano::instance::*;
use std::sync::Arc;

//para ventanas
use vulkano_win::VkSurfaceBuild;
use winit::{Event, EventsLoop, Window, WindowEvent, ControlFlow, WindowBuilder};
use winit::Icon;

//para buffer
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;

use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;


/*This funtion Creates an instace of Vulkano, if you recieve the "You Are Vulkano Ready" it means you are ready to use RustyGuiMaker, else check vulkano system requirements*/
pub fn VulkanoTest() -> &'static str{
	let _Test = super::RustyInstance();
	return "You Are Vulkano Ready";
}

pub fn CheckFamily(){
	let instance = Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");

	//let instanceStruct = Arc::new(instance);//Pass instance to struct
	let physical = PhysicalDevice::enumerate( &instance ).next().expect("no device available");

	for family in physical.queue_families() {
		println!("Found a queue family with {:?} queue(s)", family.queues_count());
	}
}

pub fn CheckFamilyWithInstance(instance : Instance){

	let instanceStruct = Arc::new(instance);//Pass instance to struct
	let physical = PhysicalDevice::enumerate( &instanceStruct ).next().expect("no device available");

	for family in physical.queue_families() {
		println!("Found a queue family with {:?} queue(s)", family.queues_count());
	}
}




pub trait CheckPhysicalDevices {
    fn answer(self);
}
pub struct withOutInstance();
pub struct withInstance(pub Instance);


impl CheckPhysicalDevices for withOutInstance {
	fn answer(self){
		let instance = Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");

		//let instanceStruct = Arc::new(instance);//Pass instance to struct
		let physical = PhysicalDevice::enumerate( &instance ).next().expect("no device available");

		for family in physical.queue_families() {
			println!("Found a queue family with {:?} queue(s)", family.queues_count());
		}
	}
}

impl CheckPhysicalDevices for withInstance {
	fn answer(self){

		let yolo = self.0;
		let instanceStruct = Arc::new(yolo);//Pass instance to struct
		let physical = PhysicalDevice::enumerate( &instanceStruct ).next().expect("no device available");

		for family in physical.queue_families() {
			println!("Found a queue family with {:?} queue(s)", family.queues_count());
		}
	}
}

pub fn overloadable<T: CheckPhysicalDevices> (arg: T) {
    arg.answer()
}



	/*events_loop.run_forever(|event| {
		match event {
			winit::Event::WindowEvent {
				event: winit::WindowEvent::CloseRequested,
				..
			} => winit::ControlFlow::Break,
			_ => winit::ControlFlow::Continue,
		}
	});*/


pub fn WindowCreation(){

	let mut events_loop = winit::EventsLoop::new();
	// let window = winit::Window::new(&events_loop).unwrap();
	let mut resizable = true;

	//let path = concat!(env!("CARGO_MANIFEST_DIR"), "/Resources/RGMLogo.png");
	//let icon = Icon::from_path(path).expect("Failed to open icon");
	//let img = image::open(concat!(env!("CARGO_MANIFEST_DIR"), "/Resources/RGMLogo.png")).unwrap();


	let window = winit::WindowBuilder::new()
		.with_title("Rusty Gui Maker.")
		//.with_window_icon(Some(icon))
		.with_dimensions((800, 800).into())
		.with_resizable(resizable)
		.build(&events_loop)
		.unwrap();

	events_loop.run_forever(|event| {
		match event {
			Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
				ControlFlow::Break
			},
			_ => ControlFlow::Continue,
		}
	});

}


pub fn WindowCreationResizeTest(){

	let mut events_loop = winit::EventsLoop::new();
	let mut resizable = true;

	let window = winit::WindowBuilder::new()
		.with_title("Hit space to toggle resizability.")
		.with_dimensions((400, 200).into())
		.with_resizable(resizable)
		.build(&events_loop)
		.unwrap();

	events_loop.run_forever(|event| {
		match event {
			winit::Event::WindowEvent { event, .. } => match event {
				winit::WindowEvent::CloseRequested => return winit::ControlFlow::Break,
				winit::WindowEvent::KeyboardInput {
					input:
						winit::KeyboardInput {
							virtual_keycode: Some(winit::VirtualKeyCode::Space),
							state: winit::ElementState::Released,
							..
						},
					..
				} => {
					resizable = !resizable;
					println!("Resizable: {}", resizable);
					window.set_resizable(resizable);
				}
				_ => (),
			},
			_ => (),
		};
		winit::ControlFlow::Continue
	});
}




pub struct MyStructtt {
	pub a: u32,
	pub b: bool,
}

pub fn BufferCreation(){
	let instance = Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");
	let physical = PhysicalDevice::enumerate( &instance ).next().expect("no device available");

	let queue_family = physical.queue_families()
		.find(|&q| q.supports_graphics())
		.expect("couldn't find a graphical queue family");

	let (device, mut queues) = {
		Device::new(physical, &Features::none(), &DeviceExtensions::none(), [ (queue_family, 0.5) ].iter().cloned()).expect("failed to create device")
	};


	//let data = 12;
	//let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), data).expect("failed to create buffer");

	//funciona pero es un buffer simple, no es un arreglo
/*
	let data = MyStructtt { a: 5, b: true };
	let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), data).unwrap();
	let mut content = buffer.write().unwrap();
	content.a *= 2;
	content.b = false;
	*/

/*	struct MyStruct {
		a: u32,
		b: bool,
	}
	let data = MyStruct { a: 5, b: true };
	let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), data).unwrap();
*/

	let iter = (0 .. 128).map(|_| MyStructtt{a: 4, b: false} );
	let buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), iter).unwrap();

	let mut content = buffer.write().unwrap();
	// `content` implements `DerefMut` whose target is of type `MyStruct` (the content of the buffer)
	content[1].a = 2;
	content[1].b = false;

	print!("{:?}", content[1].a);

/*
	let mut content = buffer.write().unwrap();
	// this time `content` derefs to `[u8]`
	content[12] = 83;
	content[7] = 3;
*/
}


/*

enum CheckPhysicalDevices {
	withOutInstance,
	withInstance(Instance),
}

//use CheckPhysicalDevices::{withOutInstance, withInstance};


fn overloadable(arg: CheckPhysicalDevices){
	match arg {
		withOutInstance => {
				let instance = Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");

				//let instanceStruct = Arc::new(instance);//Pass instance to struct
				let physical = PhysicalDevice::enumerate( &instance ).next().expect("no device available");

				for family in physical.queue_families() {
					println!("Found a queue family with {:?} queue(s)", family.queues_count());
				}
		},
		withInstance(instance) => {
				let instanceStruct = Arc::new(instance);//Pass instance to struct
				let physical = PhysicalDevice::enumerate( &instanceStruct ).next().expect("no device available");

				for family in physical.queue_families() {
					println!("Found a queue family with {:?} queue(s)", family.queues_count());
				}
		},
	}
}
*/



/*

/*check the availabe physical devices and their families*/
pub fn CheckPhysicalDevices(instance : Instance,){

	//let instance = Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");

	let instanceStruct = Arc::new(instance);//Pass instance to struct
	let physical = PhysicalDevice::enumerate( &instanceStruct ).next().expect("no device available");

	for family in physical.queue_families() {
		println!("Found a queue family with {:?} queue(s)", family.queues_count());
	}
}

/*check the availabe physical devices and their families*/
pub fn CheckPhysicalDevices(){

	let instance = Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");

	//let instanceStruct = Arc::new(instance);//Pass instance to struct
	let physical = PhysicalDevice::enumerate( &instance ).next().expect("no device available");

	for family in physical.queue_families() {
		println!("Found a queue family with {:?} queue(s)", family.queues_count());
	}
}

*/