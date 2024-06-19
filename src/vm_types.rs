
#[derive(Debug, Clone, PartialEq)]
pub enum ByteCode {
    Load(u32),
    LoadConst(u32),
    Store(u32),
    BinMul,
    BinAdd,
    BinMinus,
    BinDivide,
    Jump(u32),
    JumpIfFalse(u32),
    Call(u32),
    Cmp,
    BeginScope,
    EndScope,
    Fun(u32),
    MakeStruct,
    MakeArray(u32),
    Obj(u32),
    Assign,
    Ret(u32),
    Var(u32),
    Next,
    MakeIter,
    Await,
    AccessProp(u32),
}

impl ToString for ByteCode {
	fn to_string(&self) -> String {
		match self {
			ByteCode::Load(val) => format!("LOAD {}", val),
			ByteCode::LoadConst(val) => format!("LOAD_CONST {}", val),
			ByteCode::Store(val) => format!("STORE {}", val),
			ByteCode::BinMul => "BIN_MUL".to_string(),
			ByteCode::BinAdd => "BIN_ADD".to_string(),
			ByteCode::BinMinus => "BIN_MINUS".to_string(),
			ByteCode::BinDivide => "BIN_DIVIDE".to_string(),
			ByteCode::Jump(val) => format!("JUMP {}", val),
			ByteCode::JumpIfFalse(val) => format!("JUMP_IF_FALSE {}", val),
			ByteCode::Call(val) => format!("CALL {}", val),
			ByteCode::Cmp => "CMP".to_string(),
			ByteCode::BeginScope => "BEGIN_SCOPE".to_string(),
			ByteCode::EndScope => "END_SCOPE".to_string(),
			ByteCode::Fun(val) => format!("FUN {}", val),
			ByteCode::MakeStruct => "MAKE_STRUCT".to_string(),
			ByteCode::MakeArray(val) => format!("MAKE_ARRAY {}", val),
			ByteCode::Obj(val) => format!("OBJ {}", val),
			ByteCode::Assign => "ASSIGN".to_string(),
			ByteCode::Ret(val) => format!("RET {}", val),
			ByteCode::Var(val) => format!("VAR {}", val),
			ByteCode::Next => "NEXT".to_string(),
			ByteCode::MakeIter => "MAKE_ITER".to_string(),
			ByteCode::Await => "AWAIT".to_string(),
			ByteCode::AccessProp(val) => format!("ACCESS_PROP {}", val),
		}
	}
}