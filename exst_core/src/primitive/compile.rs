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
    //postpone
    //特殊な命令にコンパイルされるワード
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"
    : ( ")" parse document-word ; immidiate
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