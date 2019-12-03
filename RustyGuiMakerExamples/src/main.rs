#![allow(non_snake_case)]

extern crate RustyGuiMaker;
//use CheckPhysicalDevices::{withOutInstance, withInstance};
#[allow(unused)]
use RustyGuiMaker::*;
use std::io;
use std::path::Path;
//use std::process::Command;


fn main() {
	// RustyGuiMaker::Test::CheckFamily();
	loop{
		//linux
		//Command::new("clear").spawn().expect("clear failed");
		//windows
		//Command::new("cls").spawn().expect("clear failed");

		// println!("Select Example to run:");
		println!("\n\n\nSelecciona el ejemplo a correr:");
		println!("\n1: Cuadrado");

		let mut input_text = String::new();
		io::stdin().read_line(&mut input_text).expect("Error leyendo del stdin");

		let trimmed = input_text.trim();

		match trimmed.parse::<u32>() {
			Ok(i) => match i {

						1 => EjemploCuadrado(),

						//ejemplos de rangos y multiples valores
						44 | 55 | 66 => println!("Match several values"),
						50...80 => println!("Match an inclusive range"),

						// Handle the rest of cases
						_ => {
							println!("Numero fuera de rango");
						},
					},
			Err(..) => {
				println!("No es un entero: {}", trimmed);
			},
		};
	}
}


pub fn EjemploCuadrado(){
	//Primero delcaramos nuestra instancia
	let mut RGMInstance = Structs::RGMinstance::initialize();

	//Una vez declarada podemos modificar los valores por defecto
	RGMInstance.Window.SetWindowTitle(	Some( String::from("Rusty Gui maker Ejemplo Triangulo") ));
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
	RGMInstance = RustyGuiMaker::ADDFigRustyInstance(RGMInstance, Structs::Vertex::Figures::Plane, 0.1, 0.0, 0.0, "RED".to_string(), Structs::Callbacks::CallbackEmun::NON, String::from("Cuad1"));

	//una vez creadas las figuras corremos nuestra ventana
	RustyGuiMaker::UseRustyInstance(RGMInstance);
}