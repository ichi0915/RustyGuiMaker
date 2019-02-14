#![allow(non_snake_case)]

extern crate RustyGuiMaker;
//use CheckPhysicalDevices::{withOutInstance, withInstance};
use RustyGuiMaker::*;


fn main() {

	RustyGuiMaker::hello();

	let LOL = RustyGuiMaker::RustyInstance();

	println!( "{}", RustyGuiMaker::Test::VulkanoTest() );

	RustyGuiMaker::Test::CheckFamily();
	RustyGuiMaker::Test::CheckFamilyWithInstance( RustyGuiMaker::RustyInstance().instance );

	//intento de overloading
	//RustyGuiMaker::Test::withOutInstance();
	//RustyGuiMaker::Test::withInstance( LOL.instance );

}


