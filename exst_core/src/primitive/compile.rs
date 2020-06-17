//! 
//! コンパイル関連ワード
//! 

use super::super::lang::vm::*;
use super::super::lang::vm::compile::*;
use super::super::lang::value::*;
use super::util;
use std::rc::Rc;

/// コンパイル関連ワードを登録
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    vm.define_primitive_word("parse".to_string(), false, String::from("c -- str; skip to token 'c' and push skiped str"), parse);
    vm.define_primitive_word("parse-name".to_string(), false, String::from("\"name\" -- name; read next token"), parse_name);
    vm.define_primitive_word("literal".to_string(), true, String::from("a -- ; compile push a"), literal);
    vm.define_primitive_word("compile,".to_string(), false, String::from("xt -- ; compile word call from execution token"), compile);
    vm.define_primitive_word("[compile]".to_string(), true, String::from("\"word name\" -- ; compile word call"), compile_immidiate);
    vm.define_primitive_word("document-word".to_string(), false, String::from("document -- ; document to recent defined word"), document_word);
    vm.define_primitive_word("cdp".to_string(), false, String::from(" -- cdp; get code buffer address"), cdp);
    vm.define_primitive_word("__dummy_instruction__".to_string(), false, String::from(" -- ; compile instruction: dummy"), trap_dummy_instruction_execution);
    vm.define_primitive_word("__user_trap__".to_string(), false, String::from(" -- ; compile instruction: user trap"), trap_user_trap);
    vm.define_primitive_word("[__user_trap__]".to_string(), true, String::from(" -- ; compile instruction: user trap"), trap_user_trap);
    vm.define_primitive_word("__jump__".to_string(), false, String::from("adr -- ; compile instruction: jump"), jump);
    vm.define_primitive_word("__branch__".to_string(), false, String::from("adr -- ; compile instruction: branch"), branch);
    vm.define_primitive_word("__exec__".to_string(), false, String::from(" -- ; compile instruction: exec"), exec);
    vm.define_primitive_word("[__exec__]".to_string(), true, String::from(" -- ; compile instruction: exec"), exec);
    vm.define_primitive_word("__return__".to_string(), false, String::from(" -- ; compile instruction: return"), ret);
    vm.define_primitive_word("[__return__]".to_string(), true, String::from(" -- ; compile instruction: return"), ret);
    vm.define_primitive_word("instruction-at".to_string(), false, String::from("adr -- ; code_buffer[adr] = code_buffer.pop"), instruction_at);
    vm.define_primitive_word("postpone".to_string(), true, String::from("\"word name\" -- ; postpone word"), postpone);
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"
    : ( ")" parse document-word ; immediate
"#}

/// parse
fn parse<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    let end_char = top.try_into_char()?;
    let skiped = vm.input_stream_mut().skip(end_char)?;
    vm.data_stack_mut().push(Rc::new(Value::StrValue(skiped)));
    Result::Ok(())
}

/// parse_name
fn parse_name<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_with_name(vm, |v, name|{
        v.data_stack_mut().push(Rc::new(Value::StrValue(name)));
        Result::Ok(())
    })
}

/// literal
fn literal<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    vm.code_buffer_mut().push(compile_push_value(top));
    Result::Ok(())
}

/// compile,
fn compile<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    let callee: &CodeAddress = (*top).try_into()?;
    vm.code_buffer_mut().push(compile_call_code_address(*callee));
    Result::Ok(())
}

/// [compile]
fn compile_immidiate<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_with_name(vm, |v, name|{
        let w = v.word_dictionary().find_word(&name)?;
        let callee = w.code();
        v.code_buffer_mut().push(compile_call_code_address(callee));
        Result::Ok(())
    })
}

/// document-word
fn document_word<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    let doc: &String = (*top).try_into()?;
    vm.word_dictionary_mut().last_word_set_document(doc.clone());
    Result::Ok(())
}

/// cdp
fn cdp<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let here = vm.code_buffer().here();
    vm.data_stack_mut().push(Rc::new(Value::CodeAddress(here)));
    Result::Ok(())
}

/// Trap(DummyInstructionExecution)にコンパイルする
fn trap_dummy_instruction_execution<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    vm.code_buffer_mut().push(Instruction::Trap(TrapReason::DummyInstructionExecution));
    Result::Ok(())
}

/// Trap(UserTrap)にコンパイルする
fn trap_user_trap<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    vm.code_buffer_mut().push(Instruction::Trap(TrapReason::UserTrap));
    Result::Ok(())
}

/// Jumpにコンパイルする
fn jump<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    let adr = (*top).try_into()?;
    vm.code_buffer_mut().push(Instruction::Jump(*adr));
    Result::Ok(())
}

/// Branchにコンパイル
fn branch<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    let adr = (*top).try_into()?;
    vm.code_buffer_mut().push(Instruction::Branch(*adr));
    Result::Ok(())
}

/// Execにコンパイル
fn exec<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    vm.code_buffer_mut().push(Instruction::Exec);
    Result::Ok(())
}

/// Returnにコンパイル
fn ret<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    vm.code_buffer_mut().push(Instruction::Return);
    Result::Ok(())
}

/// 末尾の命令を指定された位置に書き込む
fn instruction_at<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    let adr = (*top).try_into()?;
    let inst = vm.code_buffer_mut().pop()?;
    vm.code_buffer_mut().set(*adr, inst)?;
    Result::Ok(())
}

/// postpone
fn postpone<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_with_name(vm, |v, name|{
        let w = v.word_dictionary().find_word(&name)?;
        if w.is_immediate() {
            let callee = w.code();
            v.code_buffer_mut().push(compile_call_code_address(callee));
        } else {
            let callee = w.code();
            v.code_buffer_mut().push(compile_push_value(Rc::new(Value::CodeAddress(callee))));
            v.code_buffer_mut().push(compile_call_primitive(compile));
        }
        Result::Ok(())
    })
}