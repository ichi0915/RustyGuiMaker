#![allow(non_snake_case)]

extern crate RustyGuiMaker;
//use CheckPhysicalDevices::{withOutInstance, withInstance};
use RustyGuiMaker::*;


fn main() {


	let mut Instance = RustyGuiMaker::CreateDefaultRustyInstance();
	RustyGuiMaker::UseRustyInstance(Instance);

	/*RustyGuiMaker::hello();

	let mut _LOL = RustyGuiMaker::RustyInstance();

	println!( "{}", RustyGuiMaker::Test::VulkanoTest() );

	RustyGuiMaker::Test::CheckFamily();
	RustyGuiMaker::Test::CheckFamilyWithInstance( RustyGuiMaker::RustyInstance().instance );


	RustyGuiMaker::Test::BufferCreation();

	//RustyGuiMaker::RustyInstance().WIDTH = 1000;
	//RustyGuiMaker::RustyInstance().HEIGHT = 1000;

	println!();

	println!( "{}", GetWidth( _LOL ));
	//SetWidth( &mut _LOL, 1000 );
	//_LOL = SetWidth2( _LOL, 1000 );

	println!();

	println!( "{}", GetWidth( RustyGuiMaker::RustyInstance() ));
	SetWidth( &mut RustyGuiMaker::RustyInstance(), 1000 );
	println!( "{}", GetWidth( RustyGuiMaker::RustyInstance() ));


	println!( "{}", RustyGuiMaker::RustyInstance().HEIGHT );


	//RustyGuiMaker::Test::WindowCreation();

	let mut app =  RustyGuiMaker::Structs::GuiStructApplication::initialize();
	app.main_loop();

	//intento de overloading
	//RustyGuiMaker::Test::withOutInstance();
	//RustyGuiMaker::Test::withInstance( LOL.instance );
*/
}

//https://github.com/bwasty/vulkan-tutorial-rs
//https://github.com/rukai/PF_Sandbox
//https://github.com/rukai/vulkano-text