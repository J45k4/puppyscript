use crate::vm_types::ByteCode;
use crate::PuppyScriptVM;


pub fn pretty_print_bytecode(vm: &PuppyScriptVM, blk: u32) {
	let byte_codes = vm.get_blk(blk);
	// println!("{:?}", b);

	for byte_code in byte_codes {
		match byte_code {
			ByteCode::Load(v) => println!("Load {}", vm.get_var_name(*v)),
			ByteCode::LoadConst(c) => println!("LoadConst {}", vm.get_const_name(*c)),
			ByteCode::Store(s) => println!("Store {}", vm.get_var_name(*s)),
			ByteCode::BinMul => println!("BinMul"),
			ByteCode::BinAdd => println!("BinAdd"),
			ByteCode::BinMinus => println!("BinMinus"),
			ByteCode::BinDivide => println!("BinDivide"),
			ByteCode::Jump(j) => println!("Jump {}", j),
			ByteCode::JumpIfFalse(j) => println!("JumpIfFalse {}", j),
			ByteCode::Call(a) => println!("Call {}", a),
			ByteCode::Cmp => println!("Cmp"),
			ByteCode::BeginScope => println!("BeginScope"),
			ByteCode::EndScope => println!("EndScope"),
			ByteCode::Fun(f) => println!("Fun {}", vm.get_const_name(*f)),
			ByteCode::MakeStruct => println!("MakeStruct"),
			ByteCode::MakeArray(m) => println!("MakeArray {}", m),
			ByteCode::Obj(o) => println!("Obj {}", o),
			ByteCode::Assign => println!("Assign"),
			ByteCode::Ret(r) => println!("Ret {}", r),
			ByteCode::Var(v) => println!("Var {}", v), 
			ByteCode::Next => println!("Next"),
			ByteCode::MakeIter => println!("MakeIter"),
			ByteCode::AccessProp(a) => println!("AccessProp {}", a), 
		}
	}
}