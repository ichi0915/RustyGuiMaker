#![allow(non_snake_case)]

extern crate RustyGuiMaker;
use RustyGuiMaker::*;

use std::path::Path;

fn main() {

	let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../Resources/RGMLogo.png");

	let mut WindowInstance =  Structs::RGMWindow::default();

	WindowInstance.SetWindowTitle(	Some( String::from("Rusty Gui maker") ));
	WindowInstance.SetWindowIcon(	Some( load_icon(Path::new(path)) ));
	WindowInstance.SetWidth(		Some( 800.0 )	);
	WindowInstance.SetHeight(		Some( 600.0 )	);
	WindowInstance.SetMinWidth(		Some( 400.0 )	);
	WindowInstance.SetMinHeight(	Some( 200.0 )	);
	WindowInstance.SetMaxWidth(		Some( 1024.0 )	);
	WindowInstance.SetMaxHeight(	Some( 768.0 )	);
	WindowInstance.created_at();

	//let mut Figure = String::from("");

	// let mut Figures = Structs::Vertex::Figures::addFigure();

	RustyGuiMaker::UseRustyInstance(WindowInstance);
}

//https://github.com/rukai/vulkano-text