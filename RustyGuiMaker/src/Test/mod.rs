#![allow(non_snake_case)]

/**
 ** This Source Code Form is subject to the terms of the Mozilla Public
 ** License, v. 2.0. If a copy of the MPL was not distributed with this
 ** file, You can obtain one at http://mozilla.org/MPL/2.0/.
 **
 **
 ** "Test": This File is for all the testing functions.
 **
 ** Copyright (c) "2019" Rusty Gui Maker
 **/


use vulkano::instance::*;

/*This funtion Creates an instace of Vulkano, if you recieve the "You Are Vulkano Ready" it means you are ready to use RustyGuiMaker, else check vulkano system requirements*/
pub fn VulkanoTest() -> &'static str{
	let _instance = Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");
	return "You Are Vulkano Ready";
}


/*check the availabe physical devices and their families*/
pub fn CheckPhysicalDevices(){

	let instance = Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");
	let physical = PhysicalDevice::enumerate(&instance).next().expect("no device available");

	for family in physical.queue_families() {
		println!("Found a queue family with {:?} queue(s)", family.queues_count());
	}
}


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}