use std::collections::HashMap;

use crate::callstack::*;
use crate::parsing::Parser;
use crate::scope::*;
use crate::types::*;
use crate::vm_types::*;

const PUSH_METHOD: u32 = 1;
const POP_METHOD: u32 = 2;
const MAP_METHOD: u32 = 3;
const DEL_FUN: u32 = 4;
const FIRST_IDT: u32 = 30;

fn print_stack_top(scope: &mut ScopeManager, stack: &Callstack) {
    let v = match stack.peek_value() {
        Some(v) => v.clone(),
        None => StackValue::None
    };

    match v {
        StackValue::Ptr(ptr) => {
            let val = match scope.lookup(&ptr) {
                Some(v) => v.clone(),
                None => Value::None
            };

            print!(" {:?}", val);
        },
        _ => print!(" {:?}", v)
    }
}

fn format_num(digits: u32, num: u32) -> String {
    let mut s = num.to_string();
    while s.len() < digits as usize {
        s = format!("0{}", s);
    }
    s
}

struct BuildinRes {
    call: Option<CallInstruction>,
    stack_val: Option<StackValue>,
    disable_buildin: bool
}

pub struct PuppyScriptVM {
    scopes: Vec<usize>,
    constants: Vec<Value>,
    code_blocks: Vec<Vec<ByteCode>>,
    callstacks: Vec<Callstack>,
    idt_map: HashMap<String, u32>,
    id_idt_map: HashMap<u32, String>,
    next_idt: u32,
    scope: ScopeManager,
    pub log: usize
}

impl PuppyScriptVM {
	pub fn new() -> Self {
		Self {
			scopes: Vec::new(),
            constants: Vec::new(),
            code_blocks: Vec::new(),
            callstacks: Vec::new(),
            scope: ScopeManager::new(),
            idt_map: HashMap::new(),
            id_idt_map: HashMap::new(),
            next_idt: FIRST_IDT,
            log: 0
		}
	}

	pub fn store_const(&mut self, v: Value) -> u32 {
        self.constants.push(v);
        (self.constants.len() - 1) as u32
    }

	pub fn get_const_name(&self, id: u32) -> String {
		match self.constants.get(id as usize) {
			Some(v) => format!("{:?}", v),
			None => "None".to_string()
		}
	}

    pub fn store_idt(&mut self, name: String) -> u32 {
        match name.as_str() {
            "push" => return PUSH_METHOD,
            "pop" => return POP_METHOD,
            "map" => return MAP_METHOD,
            _ => {}
        };

        match self.idt_map.get(&name) {
            Some(i) => *i,
            None => {
                let i = self.next_idt;
                self.next_idt += 1;
                self.idt_map.insert(name.clone(), i);
                self.id_idt_map.insert(i, name);
                i
            }
        }
    }

	pub fn get_var_name(&self, id: u32) -> String {
		match self.id_idt_map.get(&id) {
			Some(name) => name.clone(),
			None => format!("var_{}", id)
		}
	}

	pub fn compile_ast(&mut self, ast: &Vec<ASTNode>) -> u32 {
        let mut code_block = Vec::new();
        for node in ast {
            self.compile_node(&mut code_block, node);
        }   
        self.code_blocks.push(code_block);

        if self.log > 1 {
            println!("code blocks: {:?}", self.code_blocks);
        }

        (self.code_blocks.len() - 1) as u32
    }

    pub fn compile_code(&mut self, code: &str) -> u32 {
        let ast = Parser::new(code).parse();
        self.compile_ast(&ast)
    }

	pub fn compile_node(&mut self, block: &mut Vec<ByteCode>, node: &ASTNode) {
        if self.log > 0 {
            println!("compile: {:?}", node);
        }

        match node {
            ASTNode::Lit(lit) => {
                let i = self.store_const(lit.clone());
                block.push(ByteCode::LoadConst(i));
            },
            ASTNode::Ret(ret) => {
                let a = match &*ret.value {
                    Some(v) =>  {
                        self.compile_node(block, v);
                        1
                    },
                    None => 0,
                };

                block.push(ByteCode::Ret(a));
            },
            ASTNode::BinOp(binop) => {
                self.compile_node(block, &binop.left);
                self.compile_node(block, &binop.right);

                match binop.op {
                    Op::Plus => block.push(ByteCode::BinAdd),
                    Op::Minus => block.push(ByteCode::BinMinus),
                    Op::Mul => block.push(ByteCode::BinMul),
                    Op::Div => block.push(ByteCode::BinDivide),
                    Op::Eq => block.push(ByteCode::Cmp),
                    // Op::Neq => block.push(ByteCode::Cmp),
                    // Op::Lt => block.push(ByteCode::Cmp),
                    // Op::Gt => block.push(ByteCode::Cmp),
                    // Op::Leq => block.push(ByteCode::Cmp),
                    // Op::Geq => block.push(ByteCode::Cmp),
                    _ => panic!("Invalid operation")
                }
            },
            ASTNode::If(ifs) => {
                self.compile_node(block, &ifs.cond);
                block.push(ByteCode::JumpIfFalse(0));
                let jump_if_false_inx = block.len() - 1;

                for node in &ifs.body {
                    self.compile_node(block, node);
                }

                block[jump_if_false_inx] = ByteCode::JumpIfFalse(block.len() as u32);

                if let Some(els) = &ifs.els {
                    for node in els {
                        self.compile_node(block, node);
                    }
                }
            },
            ASTNode::Assign(a) => {
                self.compile_node(block, &a.right);
                match &*a.left {
                    ASTNode::Ident(ref name) => {
                        let i = self.store_idt(name.clone());
                        block.push(ByteCode::Store(i));
                    },
                    _ => panic!("Invalid operation")
                }
            },
            ASTNode::Ident(idt) => {
                let i = self.store_idt(idt.clone());
                block.push(ByteCode::Load(i));
            },
            ASTNode::Array(a) => {
                for item in &a.items {
                    self.compile_node(block, item);
                }
                block.push(ByteCode::MakeArray(a.items.len() as u32));
            },
            ASTNode::Fun(f) => {
                let mut fun_block = Vec::new();
                for param in &f.params {
                    let i = self.store_idt(param.name.clone());
                    fun_block.push(ByteCode::Store(i));
                }
                for node in &f.body {
                    self.compile_node(&mut fun_block, node);
                }
                self.code_blocks.push(fun_block);
                block.push(ByteCode::Fun((self.code_blocks.len() - 1) as u32));
            },
            ASTNode::Call(c) => {
                for arg in &c.args {
                    self.compile_node(block, arg);
                }

                self.compile_node(block, &c.callee);
                block.push(ByteCode::Call(c.args.len() as u32));
            },
            ASTNode::For(f) => {
                let mut start_pc = 0;
                let mut false_jump_pc = 0;
                match &f.cond {
                    ForCond::FromIt { ident, it } => {
                        self.compile_node(block, it);
                        block.push(ByteCode::MakeIter);
                        block.push(ByteCode::Next);
                        start_pc = block.len() - 1;
                        let i = self.store_idt(ident.clone());
                        block.push(ByteCode::Store(i));
                        block.push(ByteCode::Load(i));
                        block.push(ByteCode::JumpIfFalse(0));
                        false_jump_pc = block.len() - 1;
                    },
                    _ => todo!()
                }

                for node in &f.body {
                    self.compile_node(block, node);
                }

                block.push(ByteCode::Jump(start_pc as u32));
                block[false_jump_pc] = ByteCode::JumpIfFalse(block.len() as u32);
            },
            ASTNode::ObjIns(obj) => {
                // block.push(ByteCode::LoadConst(self.store_const(Value::Str(obj.name.clone()))));
                for prop in &obj.props {
                    let c = self.store_const(Value::Str(prop.name.clone()));
                    block.push(ByteCode::LoadConst(c));
                    self.compile_node(block, &prop.value);
                }
                if let Some(name) = &obj.name {
                    block.push(ByteCode::LoadConst(self.store_const(Value::Str(name.clone()))));
                } else {
                    block.push(ByteCode::LoadConst(self.store_const(Value::None)));
                }
                block.push(ByteCode::Obj(obj.props.len() as u32));
            },
            ASTNode::ProbAccess(a) => {
                self.compile_node(block, &a.object);
                let i = self.store_idt(a.property.clone());
                block.push(ByteCode::AccessProp(i));
            },
            _ => todo!("{:?}", node)
        }
    }

	pub fn cont(&mut self, stack_id: usize, value: Value) -> RunResult {
        let stack = self.callstacks.get_mut(stack_id).unwrap();
        let val = match value {
            Value::Int(i) => StackValue::Int(i),
            Value::Float(f) => StackValue::Float(f),
            Value::Str(s) => StackValue::Str(s),
            Value::Bool(b) => StackValue::Bool(b),
            _ => todo!("{:?}", value)
        };
        stack.push_value(val);
        self.run_stack(stack_id)
    }

	pub fn run_stack(&mut self, stack_id: usize) -> RunResult {
		if self.log > 0 {
            println!("run stack: {}", stack_id);
        }
        
        loop {
            let stack = match self.callstacks.get_mut(stack_id) {
                Some(s) => s,
                None => {
                    if self.log > 0 {
                        println!("no stack");
                    }
                    return RunResult::None;
                }
            };

            if stack.depth() == 0 {
                if self.log > 0 {
                    println!("no stack");
                }
                return RunResult::None;
            }

            if self.log > 1 {
                println!("stack: {:?}", stack);
            }

            loop {
                let scope_id = stack.scope_id();

                let buidin_res = match stack.get_buildin() {
                    BuildIn::Map { ptr, inx, blk } => {
                        if self.log > 0 {
                            let m = match self.scope.lookup(&ptr) {
                                Some(v) => format!("{:?}", v),
                                None => "None".to_string()
                            };
                            print!("map: {} inx: {} blk: {}", m, inx, blk);
                        }
                        
                        let p = {
                            let val = match self.scope.lookup(&ptr) {
                                Some(v) => v,
                                None => todo!()
                            };

                            let i = *inx as usize;
                            match val {
                                Value::List(list) => {
                                    match list.get(i) {
                                        Some(v) => StackValue::from(v),
                                        None => {
                                            print!(" val: None");
                                            StackValue::None
                                        }
                                    }
                                },
                                _ => todo!("{:?}", val)
                            }
                        };
                        let ret = match p {
                            StackValue::None => BuildinRes {
                                call: None,
                                disable_buildin: true,
                                stack_val: Some(StackValue::Ptr(ptr.clone()))
                            },
                            _ => {
                                let scope_id = self.scope.create_child_scope(scope_id);
    
                                let args = vec![
                                    StackValue::Int(*inx as i64),
                                    p,
                                ];

                                if self.log > 0 {
                                    print!(" args: {:?}", args);
                                }

                                BuildinRes {
                                    call: Some(CallInstruction {
                                        blk: *blk,
                                        scope_id,
                                        values: args,
                                        ..Default::default()
                                    }),
                                    disable_buildin: false,
                                    stack_val: None
                                }
                            }
                        };
                        if self.log > 0 {
                            println!();
                        }
                        ret
                    },
                    BuildIn::None => BuildinRes { 
                        call: None, 
                        stack_val: None, 
                        disable_buildin: false 
                    }
                };

                if buidin_res.disable_buildin {
                    stack.set_buildin(BuildIn::None);
                }

                if let Some(call) = buidin_res.call {
                    stack.push(call);
                }

                if let Some(v) = buidin_res.stack_val {
                    stack.push_value(v);
                }

                let mut curr_blk = match stack.blk() {
                    Some(b) => b,
                    None => {
                        if self.log > 0 {
                            println!("no blk");
                        }
                        return RunResult::None;
                    }
                };

                if stack.pc() >= self.code_blocks[curr_blk as usize].len() as u32  {
                    break;
                }   

                let pc = stack.pc();
                stack.increment_pc();

                let c = &self.code_blocks[curr_blk as usize][pc as usize];

                if self.log > 0 {
                    print!("blk: {} pc: {}, code: {:?}", curr_blk, format_num(5, pc), c);

                    match c {
                        ByteCode::Load(i) => {
                            print!(" {}", self.id_idt_map.get(&i).unwrap());
                            print_stack_top(&mut self.scope, stack);
                        },
                        ByteCode::Store(i) => {
                            print!(" {}", self.id_idt_map.get(&i).unwrap());
                            print_stack_top(&mut self.scope, stack);
                        },
                        ByteCode::JumpIfFalse(_) => print_stack_top(&mut self.scope, stack),
                        ByteCode::Next => print_stack_top(&mut self.scope, stack),
                        ByteCode::LoadConst(i) => print!(" {:?}", self.constants[*i as usize].clone()),
                        ByteCode::Ret(_) => print_stack_top(&mut self.scope, stack),
                        ByteCode::Call(_) => print_stack_top(&mut self.scope, stack),
                        _ => {}
                    }

                    println!("");
                }

                match c {
                    ByteCode::Load(i) => {
                        match self.scope.lookup(&Ptr{ scope_id: stack.scope_id(), id: *i}) {
                            Some(v) => stack.push_value(StackValue::from(&*v)),
                            None => stack.push_value(StackValue::Undef(*i))
                        };
                    },
                    ByteCode::Store(i) => {
                        let v = Value::from(stack.pop_value().unwrap());
                        self.scope.store_named(stack.scope_id(), *i, v);
                    },
                    ByteCode::BinMul |
                    ByteCode::BinAdd |
                    ByteCode::BinMinus |
                    ByteCode::BinDivide => {
                        let tos = stack.pop_value().unwrap();
                        let tos1 = stack.pop_value().unwrap();

                        let v = match (tos1,tos) {
                            (StackValue::Int(a), StackValue::Int(b)) => {
                                match c {
                                    ByteCode::BinMul => StackValue::Int(a * b),
                                    ByteCode::BinAdd => StackValue::Int(a + b),
                                    ByteCode::BinMinus => StackValue::Int(a - b),
                                    ByteCode::BinDivide => StackValue::Int(a / b),
                                    _ => panic!("Invalid operation")
                                }
                            },
                            (StackValue::Float(a), StackValue::Float(b)) => {
                                match c {
                                    ByteCode::BinMul => StackValue::Float(a * b),
                                    ByteCode::BinAdd => StackValue::Float(a + b),
                                    ByteCode::BinMinus => StackValue::Float(a - b),
                                    ByteCode::BinDivide => StackValue::Float(a / b),
                                    _ => panic!("Invalid operation")
                                }
                            },
                            (StackValue::Float(a), StackValue::Int(b)) => {
                                match c {
                                    ByteCode::BinMul => StackValue::Float(a * b as f64),
                                    ByteCode::BinAdd => StackValue::Float(a + b as f64),
                                    ByteCode::BinMinus => StackValue::Float(a - b as f64),
                                    ByteCode::BinDivide => StackValue::Float(a / b as f64),
                                    _ => panic!("Invalid operation")
                                }
                            },
                            (StackValue::Int(a), StackValue::Float(b)) => {
                                match c {
                                    ByteCode::BinMul => StackValue::Float(a as f64 * b),
                                    ByteCode::BinAdd => StackValue::Float(a as f64 + b),
                                    ByteCode::BinMinus => StackValue::Float(a as f64 - b),
                                    ByteCode::BinDivide => StackValue::Float(a as f64 / b),
                                    _ => panic!("Invalid operation")
                                }
                            },
                            _ => panic!("Invalid operation")
                        };

                        stack.push_value(v);
                    },
                    ByteCode::Jump(indx) => stack.set_pc(*indx),
                    ByteCode::JumpIfFalse(inx) => {
                        let v = stack.pop_value().unwrap();

                        match v {
                            StackValue::Bool(b) => {
                                if !b {
                                    stack.set_pc(*inx);
                                }
                            },
                            StackValue::None => {
                                stack.set_pc(*inx);
                            },
                            StackValue::Int(i) => {
                                if i < 1 {
                                    stack.set_pc(*inx);
                                } 
                            },
                            StackValue::Ptr(ptr) => {
                                let val = match self.scope.lookup(&ptr) {
                                    Some(v) => v,
                                    None => todo!()
                                };

                                match val {
                                    Value::List(arr) => {
                                        if arr.len() < 1 {
                                            stack.set_pc(*inx);
                                        }
                                    },
                                    _ => todo!("{:?}", val)
                                };
                            },
                            _ => panic!("{:?}", v)
                        }
                    },
                    ByteCode::Call(arg_count) => {
                        if self.log > 1 {
                            println!("{:?}", stack);
                        }

                        let callee = stack.pop_value().unwrap();

                        let mut args = Vec::new();

                        for _ in 0..*arg_count {
                            let v = stack.pop_value().unwrap();
                            args.push(v);
                        }   

                        args.reverse();

                        if self.log > 1 {
                            println!("args: {:?}", args);
                        }

                        match callee {
                            StackValue::Fn(blk) => {
                                let scope_id = self.scope.create_child_scope(stack.scope_id());

                                stack.push(CallInstruction {
                                    blk,
                                    scope_id,
                                    values: args,
                                    ..Default::default()
                                });

                                curr_blk = blk;
                            },
                            StackValue::Undef(i) => {
								// let undef_call = StackValue::UndefCall {
                                //     ident: i,
                                //     args
                                // };
								// if self.log > 1 {
								// 	println!("undef call: {:?}", undef_call);
								// }
                                return RunResult::Call { 
									stack_id, 
									ident: i,
									args: args.iter().map(|v| Value::from(v)).collect(),
								}
                            },
                            StackValue::PropAccess { ptr, prop } => {
                                let val = match self.scope.lookup(&ptr) {
                                    Some(v) => v,
                                    None => todo!()
                                };

                                match val {
                                    Value::List(l) => {
                                        match prop {
                                            PUSH_METHOD => {
                                                for arg in args {
                                                    l.push(Value::from(arg));
                                                }
                                            },
                                            POP_METHOD => {
                                                let v = l.pop().unwrap();
                                                stack.push_value(StackValue::from(&v));
                                            },
                                            MAP_METHOD => {
                                                if args.len() < 1 {
                                                    todo!()
                                                }
                                                let val = &args[0];
                                                match val {
                                                    StackValue::Fn(blk) => {
                                                        stack.set_buildin(
                                                            BuildIn::Map {
                                                                blk: *blk,
                                                                ptr: ptr,
                                                                inx: 0
                                                            }
                                                        );
                                                    },
                                                    _ => todo!("{:?}", val)
                                                }

                                                continue;
                                            },
                                            _ => todo!()
                                        }
                                    },
                                    _ => todo!("{:?}", val)
                                };
                            },
                            _ => todo!("invalid callee {:?}", callee)
                        }
                    },
                    ByteCode::Cmp => {
                        let tos = stack.pop_value().unwrap();
                        let tos1 = stack.pop_value().unwrap();
                        
                        let v = match (tos, tos1) {
                            (StackValue::Int(a), StackValue::Int(b)) => StackValue::Bool(a == b),
                            (StackValue::Float(a), StackValue::Float(b)) => StackValue::Bool(a == b),
                            (StackValue::Float(a), StackValue::Int(b)) => StackValue::Bool(a == b as f64),
                            (StackValue::Int(a), StackValue::Float(b)) => StackValue::Bool(a as f64 == b),
                            (StackValue::Bool(a), StackValue::Bool(b)) => StackValue::Bool(a == b),
                            (StackValue::Str(a), StackValue::Str(b)) => StackValue::Bool(a == b),
                            _ => panic!("Invalid operation")
                        };

                        stack.push_value(v);
                    },
                    ByteCode::BeginScope => {
                        let scope_id = self.scope.create_child_scope(stack.scope_id());
                        stack.set_scope_id(scope_id);
                    },
                    ByteCode::EndScope => {
                        let parent_scope = self.scope.get_parent_scope(stack.scope_id()).unwrap();
                        stack.set_scope_id(parent_scope);
                    },
                    ByteCode::LoadConst(a) => {
                        let v = self.constants[*a as usize].clone();
                        let v = StackValue::from(&v);
                        stack.push_value(v);
                    },
                    ByteCode::MakeStruct => todo!(),
                    ByteCode::MakeArray(len) => {
                        let mut items = vec![];
                        for _ in 0..*len {
                            let v = Value::from(stack.pop_value().unwrap());
                            items.push(v);
                        }
                        items.reverse();
                        let ptr = self.scope.store_unamed(scope_id, Value::List(items));
                        if self.log > 1 {
                            println!("make array ptr: {:?}", ptr);
                        }
                        stack.push_value(StackValue::Ptr(ptr));
                    },
                    ByteCode::Assign => todo!(),
                    ByteCode::Ret(_) => {
                        if self.log > 1 {
                            println!("stack: {:?}", stack);
                            println!("stack depth: {}", stack.depth());
                        }

                        if stack.depth() > 1 {
                            let v = match stack.pop_value() {
                                Some(v) =>  {
                                    match v {
                                        StackValue::Ptr(ptr) => {
                                            let parent_scope = self.scope.get_parent_scope(scope_id).unwrap();
                                            let ptr = self.scope.move_to(&ptr, parent_scope);
                                            StackValue::Ptr(ptr)
                                        },
                                        _ => v
                                    }
                                },
                                None => StackValue::None
                            };  
                            stack.pop();
                            stack.push_value(v);
                        } else {
                            return match stack.pop_value() {
                                Some(v) => {
                                    let value = self.scope.copy_value(Value::from(v));
                                    RunResult::Value(value)
                                },
                                None => RunResult::None
                            };
                        }
                    },
                    ByteCode::Fun(i) => stack.push_value(StackValue::Fn(*i)),
                    ByteCode::Next => {
                        let val = stack.peek_value().unwrap();
                        let ptr = match val {
                            StackValue::Ptr(ptr) => ptr.clone(),
                            _ => todo!("{:?}", val)
                        };

                        if self.log > 1 {
                            println!("next: {:?}", ptr);
                        }

                        let val = {
                            let val = match self.scope.lookup(&ptr) {
                                Some(v) => v,
                                None => todo!()
                            };

                            let it = match val {
                                Value::ListIter(it)=> it.clone(),
                                _ => todo!("{:?}", val)
                            };
                            let val = match self.scope.lookup(&it.ptr) {
                                Some(v) => v,
                                None => todo!()
                            };

                            let v = match val {
                                Value::List(arr) => {
                                    match arr.get(it.inx as usize) {
                                        Some(v) => v,
                                        None => &Value::None
                                    }
                                },
                                _ => todo!("{:?}", val)
                            };
                            StackValue::from(v)
                        };

                        if let StackValue::None = val {
                            stack.pop_value();
                        } else {
                            if let Some(Value::ListIter(it)) = self.scope.lookup(&ptr) {
                                it.inx += 1
                            }
                        }

                        stack.push_value(val);
                    },
                    ByteCode::MakeIter => {
                        let val = stack.pop_value().unwrap();

                        match val {
                            StackValue::Ptr(ptr) => {
                                let val = match self.scope.lookup(&ptr) {
                                    Some(v) => v,
                                    None => todo!()
                                };

                                match val {
                                    Value::List(list) => {
                                        let ptr = self.scope.store_unamed(scope_id, Value::ListIter(
                                            ListIter {
                                                ptr,
                                                inx: 0
                                            }
                                        ));
                                        stack.push_value(StackValue::Ptr(ptr));
                                    },
                                    _ => todo!("{:?}", val)
                                };
                            },
                            _ => todo!("{:?}", val)
                        }
                    },
                    ByteCode::Obj(arg_count) => {
                        let name = match stack.pop_value() {
                            Some(v) => match v {
                                StackValue::Str(s) => Some(s),
                                _ => None
                            }, 
                            None => None
                        };
                        
                        let mut obj = Obj {
                            name,
                            props: vec![]
                        };

                        for _ in 0..*arg_count {
                            let v = stack.pop_value().unwrap();
                            let k = stack.pop_value().unwrap();

                            let key = match k {
                                StackValue::Str(s) => s,
                                _ => todo!("{:?}", k)
                            };

                            obj.props.push(
                                ObjProp {
                                    name: key,
                                    value: Value::from(v)
                                }
                            );
                        }

                        if self.log > 1 {
                            println!("obj: {:?}", obj);
                        }

                        let ptr = self.scope.store_unamed(scope_id, Value::Obj(obj));

                        if self.log > 1 {
                            println!("obj ptr: {:?}", ptr);
                        }

                        stack.push_value(StackValue::Ptr(ptr));
                    },
                    ByteCode::AccessProp(a) => {
                        let val = stack.pop_value().unwrap();
                        match val {
                            StackValue::Ptr(ptr) => {
                                stack.push_value(StackValue::PropAccess { ptr: ptr.clone(), prop: *a });
                            },
                            _ => todo!("{:?}", val)
                        };
                    },
                    _ => todo!("{:?}", c)
                };

                let tos = match stack.get_buildin() {
                    BuildIn::None => StackValue::None,
                    _ => match stack.pop_value() {
                        Some(v) => v,
                        None => StackValue::None
                    }
                };

                let scope_id = stack.scope_id();
                match stack.get_buildin() {
                    BuildIn::Map { ptr, inx, blk } => {
                        let val = match self.scope.lookup(&ptr) {
                            Some(v) => v,
                            None => todo!()
                        };

                        match val {
                            Value::List(list) => list[*inx as usize] = Value::from(tos),
                            _ => todo!("{:?}", val)
                        };

                        *inx += 1;
                    },
                    BuildIn::None => {}
                }
            }

            stack.pop();
            if self.log > 0 {
                println!("stack popped");
            }

            if self.log > 1 {
                println!("callstacks: {:?}", self.callstacks);
            } 
        }		
	}

	pub fn run_code(&mut self, code: &str) -> RunResult {
        let ast = Parser::new(code).parse();
        let blk = self.compile_ast(&ast);

        if self.log > 0 {
            println!("compiled ast to blk: {}", blk);
        }

        self.run_blk(blk, Value::None)
	}

	pub fn run_blk(&mut self, blk: u32, args: Value) -> RunResult {
        if self.log > 0 {
            println!("run_blk blk: {} args: {:?}", blk, args);
        }

        let scope_id = self.scope.create_scope();

        let mut stack = Callstack::new();
        stack.log = self.log;
        stack.push(CallInstruction {
            blk: blk,
            scope_id,
            ..Default::default()
        });
        self.callstacks.push(stack);

        if self.log > 1 {
            println!("callstacks: {:?}", self.callstacks);
        }

        self.run_stack(self.callstacks.len() - 1)
    }

	pub fn get_val(&mut self, ptr: &Ptr) -> Option<&mut Value> {
        self.scope.lookup(&ptr)
    }

	pub fn get_blk(&self, blk: u32) -> &Vec<ByteCode> {
		&self.code_blocks[blk as usize]
	}
}