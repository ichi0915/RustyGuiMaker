#![allow(non_snake_case)]

extern crate RustyGuiMaker;

fn main() {

	RustyGuiMaker::hello();

	println!( "{}", RustyGuiMaker::VulkanoTest() );

	RustyGuiMaker::CheckPhysicalDevices();
}


