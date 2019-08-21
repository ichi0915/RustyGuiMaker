#![allow(non_snake_case)]

extern crate RustyGuiMaker;
//use CheckPhysicalDevices::{withOutInstance, withInstance};
use RustyGuiMaker::*;


fn main() {

	RustyGuiMaker::hello();

	let _LOL = RustyGuiMaker::RustyInstance();

	println!( "{}", RustyGuiMaker::Test::VulkanoTest() );

	RustyGuiMaker::Test::CheckFamily();
	RustyGuiMaker::Test::CheckFamilyWithInstance( RustyGuiMaker::RustyInstance().instance );

	RustyGuiMaker::Test::WindowCreation();


	RustyGuiMaker::Test::BufferCreation();


	//intento de overloading
	//RustyGuiMaker::Test::withOutInstance();
	//RustyGuiMaker::Test::withInstance( LOL.instance );

}

//https://github.com/bwasty/vulkan-tutorial-rs
//https://github.com/rukai/PF_Sandbox
//https://github.com/rukai/vulkano-text