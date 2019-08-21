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



use vulkano::instance::*;
use std::sync::Arc;


pub mod Test;
pub mod Structs;

pub fn hello() {
	println!("Rusty GUI Maker is alive!");
}

pub fn RustyInstance() -> Structs::GuiStruct{
	//let isntaceCrea: Structs::GuiStruct = Structs::GuiStruct { instance: std::sync::Arc::<Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance")>, x: 0.4 };

	let vulkanInstance: Structs::GuiStruct = Structs::GuiStruct {
		instance: Arc::try_unwrap(Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance")).unwrap(),
		x: 0.4
	};

	return vulkanInstance;
}


//cargo doc --open

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
