//! 
//! ワード定義関連ワード
//! 

use super::super::lang::vm::*;
use super::super::lang::vm::compile::*;
use super::super::lang::word::*;
use super::super::lang::value::*;
use super::util;
use std::rc::Rc;

/// ワード定義関連ワードを登録
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    vm.define_primitive_word(":".to_string(), false, String::from("\"word name\" -- ; define word"), colon);
    vm.define_primitive_word(":noname".to_string(), false, String::from(" -- ; define anonimous word"), noname);
    vm.define_primitive_word(":recursive".to_string(), false, String::from("\"word name\" -- ; define recursible word"), recursive);
    vm.define_primitive_word(";".to_string(), true, String::from(" -- ; word definition terminator"), semicolon);
    vm.define_primitive_word("immidiate".to_string(), false, String::from(" -- ; change to immidiate word"), immidiate);
    vm.define_primitive_word("defer".to_string(), false, String::from("\"word name\" -- ; defer word"), defer);
    vm.define_primitive_word("'".to_string(), false, String::from("\"word name\" -- xt; push execution token"), tick);
    vm.define_primitive_word("[']".to_string(), true, String::from("\"word name\" -- xt; push execution token"), tick);
    vm.define_primitive_word("is".to_string(), false, String::from("xt \"word name\" -- ; set execution token to defer word"), is);
    //execute
    vm.define_primitive_word("create".to_string(), false, String::from("\"word name\" -- ; create with data buffer"), create);
    //does>
    vm.define_primitive_word("recurse".to_string(), true, String::from(" -- ; recursive call"), recurse);
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"

"#}

/// コロン定義
fn colon<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_with_name(vm, |v, name|{
        let here = v.code_buffer().here();
        v.word_dictionary_mut().reserve_word_def(name, Word::new(here));
        v.set_state(VmState::Compilation);
        Result::Ok(())
    })
}

/// 無名ワード定義
fn noname<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let here = vm.code_buffer().here();
    vm.data_stack_mut().push(Rc::new(Value::CodeAddress(here)));
    vm.set_state(VmState::Compilation);
    Result::Ok(())
}

/// 再帰ワード定義
fn recursive<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_with_name(vm, |v, name|{
        let here = v.code_buffer().here();
        v.word_dictionary_mut().reserve_word_def(name, Word::new(here));
        v.set_state(VmState::RecursableCompilation);
        Result::Ok(())
    })
}

/// セミコロン定義
fn semicolon<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    vm.code_buffer_mut().push(compile_return());
    vm.code_buffer_mut().push(compile_word_terminator());
    vm.word_dictionary_mut().complate_word_def()?;
    vm.set_state(VmState::Interpretation);
    Result::Ok(())
}

/// immidiate
fn immidiate<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    vm.word_dictionary_mut().last_word_change_immidiate();
    Result::Ok(())
}

/// defer
fn defer<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_with_name(vm, |v, name|{
        let here = v.code_buffer().here();
        v.word_dictionary_mut().reserve_word_def(name, Word::new(here));
        v.code_buffer_mut().push(compile_dummy_instruction());
        v.code_buffer_mut().push(compile_return());
        v.code_buffer_mut().push(compile_word_terminator());
        v.word_dictionary_mut().complate_word_def()?;
        Result::Ok(())
    })
}

/// tick
fn tick<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_with_name(vm, |v, name|{
        let w = v.word_dictionary().find_word(&name)?;
        let adr = w.code();
        v.data_stack_mut().push(Rc::new(Value::CodeAddress(adr)));
        Result::Ok(())
    })
}

/// is
fn is<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_with_name(vm, |v, name|{
        let w = v.word_dictionary().find_word(&name)?;
        let adr = w.code();
        let top = v.data_stack_mut().pop()?;
        let callee: &CodeAddress = (*top).try_into()?;
        v.code_buffer_mut().set(adr, compile_call_code_address(*callee))?;
        Result::Ok(())
    })
}

/// create
fn create<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_with_name(vm, |v, name|{
        let here = v.code_buffer().here();
        let data = v.data_buffer().here();
        v.word_dictionary_mut().reserve_word_def(name, Word::new(here));
        v.code_buffer_mut().push(compile_push_value(Rc::new(Value::DataAddress(data))));
        v.code_buffer_mut().push(compile_return());
        v.code_buffer_mut().push(compile_word_terminator());
        v.word_dictionary_mut().complate_word_def()?;
        Result::Ok(())
    })
}

/// 再帰呼び出し
fn recurse<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let w = vm.word_dictionary().find_last_word()?;
    let callee = w.code();
    vm.code_buffer_mut().push(compile_call_code_address(callee));
    Result::Ok(())
}