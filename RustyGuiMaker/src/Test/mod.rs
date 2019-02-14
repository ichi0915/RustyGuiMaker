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


use vulkano::instance::*;
use std::sync::Arc;


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


/*

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

*/



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