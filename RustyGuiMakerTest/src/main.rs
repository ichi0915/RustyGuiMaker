#![allow(non_snake_case)]

extern crate RustyGuiMaker;

fn main() {

	RustyGuiMaker::hello();


	println!( "{}", RustyGuiMaker::Test::VulkanoTest() );


	RustyGuiMaker::Test::CheckPhysicalDevices();
}


