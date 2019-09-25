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
//para vulkano y rust
use vulkano::instance::*;
#[allow(unused)]
use std::sync::Arc;

//para ventanas
#[allow(unused)]
use vulkano_win::VkSurfaceBuild;
#[allow(unused)]
use winit::{Event, EventsLoop, Window, WindowEvent, ControlFlow, WindowBuilder};
#[allow(unused)]
use winit::Icon;

//para buffer
#[allow(unused)]
use vulkano::buffer::BufferUsage;
#[allow(unused)]
use vulkano::buffer::CpuAccessibleBuffer;

#[allow(unused)]
use vulkano::device::Device;
#[allow(unused)]
use vulkano::device::DeviceExtensions;
#[allow(unused)]
use vulkano::device::Features;


pub fn CheckFamily(){
	let instance = Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");

	//let instanceStruct = Arc::new(instance);//Pass instance to struct
	let physical = PhysicalDevice::enumerate( &instance ).next().expect("no device available");

	for family in physical.queue_families() {
		println!("Found a queue family with {:?} queue(s)", family.queues_count());
	}
}
