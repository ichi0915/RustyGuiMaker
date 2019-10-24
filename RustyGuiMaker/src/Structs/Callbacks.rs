//https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust

#[derive(Debug, Clone)]
pub enum CallbackEmun {
	ADD,
	DEL,
	MOD,
	CSM,
	NON
}
impl CallbackEmun {
	pub fn as_str(&self) -> &'static str {
		match *self {
			CallbackEmun::ADD => "ADD",
			CallbackEmun::DEL => "DEL",
			CallbackEmun::MOD => "MOD",
			CallbackEmun::CSM => "CSM",
			CallbackEmun::NON => "NON"
		}
	}

}

#[derive(Debug, Clone)]
pub struct Processor<CB> where CB: FnMut() {
	pub callback: CB,
	pub callbackQueue: Vec<Option<CallbackEmun>>
}

#[allow(unused_parens)]
impl<CB> Processor<CB> where CB: FnMut() {
	pub fn set_callback(&mut self, c: CB) {
		self.callback = c;
	}

	pub fn process_events(&mut self, funcion: CallbackEmun) {

		let funcionstr = CallbackEmun::as_str(&funcion);

		if(funcionstr == "ADD" ){
			self.AddFigToVec();
		}
		else if(funcionstr == "DEL" ){
			self.DelFigToVec();
		}
		else if(funcionstr == "MOD" ){
			self.ModFigToVec();
		}
		else if(funcionstr == "CSM" ){
			self.CsmFigToVec();
		}
		else if(funcionstr == "NON" ){
		}
		else{
			//Imposible
			println!("Funcion no reconocida");
			(self.callback)();
		}
	}

	pub fn AddFigToVec(&mut self) {
		let mut stack = self.callbackQueue.clone();
		stack.push( Some(CallbackEmun::ADD) );

		self.callbackQueue = stack;
		println!("ADD",);
	}

	pub fn DelFigToVec(&mut self) {
		let mut stack = self.callbackQueue.clone();
		stack.push( Some(CallbackEmun::DEL) );

		self.callbackQueue = stack;
		println!("DEL",);
	}

	pub fn ModFigToVec(&mut self) {
		println!("MOD",);
	}

	pub fn CsmFigToVec(&mut self) {
		println!("CSM",);
	}

	pub fn GetQueue(&mut self) -> Vec<Option<CallbackEmun>> {
		return self.callbackQueue.clone();
	}

	pub fn PrintQueue(&mut self) {
		for d in &mut self.callbackQueue.clone() {
			if let Some(ref mut du) = *d {
				//Sin some
				println!(" Print:  {:?}", du);
			}
			//Con some
			// println!(" Print:  {:?}", d);
		}
	}

	pub fn CleanQueue(&mut self)  {
		self.callbackQueue = Vec::new( );
	}

}

