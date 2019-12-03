#![allow(non_snake_case)]

extern crate RustyGuiMaker;
use RustyGuiMaker::*;

use std::path::Path;

fn main() {

	let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../Resources/RGMLogo.png");

	//Primero delcaramos nuestra instancia
	let mut RGMInstance = Structs::RGMinstance::initialize();

	//Una vez declarada podemos modificar los valores por defecto
	RGMInstance.Window.SetWindowTitle(	Some( String::from("Rusty Gui maker") ));
	RGMInstance.Window.SetWindowIcon(	Some( load_icon(Path::new(path)) ));
	RGMInstance.Window.SetWidth(		Some( 800.0 )	);
	RGMInstance.Window.SetHeight(		Some( 600.0 )	);
	RGMInstance.Window.SetMinWidth(		Some( 400.0 )	);
	RGMInstance.Window.SetMinHeight(	Some( 200.0 )	);
	RGMInstance.Window.SetMaxWidth(		Some( 1024.0 )	);
	RGMInstance.Window.SetMaxHeight(	Some( 768.0 )	);
	RGMInstance.Window.created_at();

	//Despues de modificar los valores por defecto(no es requerido modificarlos) inicializamos nuestra instancia
	RGMInstance = RustyGuiMaker::StartRustyInstance(RGMInstance);

	//Ya creada e inicializada podemos empezar a anadir figuras
	//En este ejemplo vamos a crear 2 figuras de nuestra base y 2 Customizadas
	RGMInstance = RustyGuiMaker::ADDFigRustyInstance(RGMInstance, Structs::Vertex::Figures::Plane, 0.1, 0.0, 0.0, "RED".to_string(), Structs::Callbacks::CallbackEmun::ADD, String::from("Cuad1"));
	RGMInstance = RustyGuiMaker::ADDFigRustyInstance(RGMInstance,Structs::Vertex::Figures::Triangle, 0.1, 0.5, 0.0, "YELLOW".to_string(), Structs::Callbacks::CallbackEmun::DEL, String::from("Trian1"));

	//figura cstm1
	let mut CSTMVert = Vec::new();
	CSTMVert.push(Structs::Vertex::Points { Position: [-1.0,-1.0, 0.0], });
	CSTMVert.push(Structs::Vertex::Points { Position: [ 1.0,-1.0, 0.0], });
	CSTMVert.push(Structs::Vertex::Points { Position: [-1.0, 1.0, 0.0], });
	CSTMVert.push(Structs::Vertex::Points { Position: [ 1.0,-1.0, 0.0], });
	CSTMVert.push(Structs::Vertex::Points { Position: [-1.0, 1.0, 0.0], });
	CSTMVert.push(Structs::Vertex::Points { Position: [ 1.0, 1.0, 0.0], });
	RGMInstance = RustyGuiMaker::ADDCSTMFigRustyInstance(RGMInstance, CSTMVert, 0.1, -0.5, 0.0, "PURPLE".to_string(), Structs::Callbacks::CallbackEmun::NON , String::from("CSTMFIG"));

	//figura cstm2
	let mut CSTMVert2 = Vec::new();
	CSTMVert2.push(Structs::Vertex::Points { Position: [-1.0,-1.0, 0.0], });
	CSTMVert2.push(Structs::Vertex::Points { Position: [ 1.0,-1.0, 0.0], });
	CSTMVert2.push(Structs::Vertex::Points { Position: [-1.0, 1.0, 0.0], });
	RGMInstance = RustyGuiMaker::ADDCSTMFigRustyInstance(RGMInstance, CSTMVert2, 0.1, -0.5, 0.5, "GREEN".to_string(), Structs::Callbacks::CallbackEmun::NON , String::from("CSTMFIG2"));

	//una vez creadas las figuras corremos nuestra ventana
	RustyGuiMaker::UseRustyInstance(RGMInstance);
}

//https://github.com/rukai/vulkano-text