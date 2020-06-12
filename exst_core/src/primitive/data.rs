//! 
//! データ操作関連ワード
//! 

use super::super::lang::vm::*;
use super::super::lang::value::*;
use super::super::lang::word::*;
use super::super::lang::vm::compile::*;
use super::util;

/// データ操作関連ワードを登録
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    vm.define_primitive_word("allot".to_string(), false, String::from("size -- ; allocate data buffer space"), allot);
    vm.define_primitive_word(",".to_string(), false, String::from("value -- ; create new data buffer space and set value"), create_one_cell);
    vm.define_primitive_word("@".to_string(), false, String::from("adr -- value; get value from adr"), get);
    vm.define_primitive_word("!".to_string(), false, String::from("value adr -- ; set value to adr"), set);
    vm.define_primitive_word("body>".to_string(), false, String::from("xt -- adr; get data address from execution token"), body);
    vm.define_primitive_word("constant".to_string(), false, String::from("value \"variable\" -- ; set value to Value Variable"), constant);
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"
    : variable ( "variable name" -- ; create variable )
        create 1 allot
    ;
    : value ( value "value name" -- ; create value )
        create , does> @
    ;
    : +! ( value variable -- ; add value to variable )
        dup @ rot + swap !
    ;
    : -! ( value variable --; substruct value from variable )
        dup @ rot - swap !
    ;
    : -> ( value "variable" -- ; set value to Value Variable )
        ' body> !
    ;
    : ++> ( value "Value" -- ; add value to Value)
        ' body> +!
    ;
    : --> ( value "Value" -- ; subtruct value from Value )
        ' body> -!
    ;
"#}

/// allot
fn allot<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    let size = (*top).try_into_usize()?;
    vm.data_buffer_mut().allocate(size);
    Result::Ok(())
}

/// ,
fn create_one_cell<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    let adr = vm.data_buffer().here();
    vm.data_buffer_mut().allocate(1);
    vm.data_buffer_mut().set(adr, top)?;
    Result::Ok(())
}

/// @
fn get<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    match *top {
        Value::DataAddress(adr) => {
            let value = vm.data_buffer().get(adr)?;
            vm.data_stack_mut().push(value);
            Result::Ok(())
        },
        Value::EnvAddress(adr) => {
            let base = vm.return_stack().peek()?;
            let value = vm.env_stack_mut().get(base.stack_address(), adr)?;
            vm.data_stack_mut().push(value);
            Result::Ok(())
        },
        _ => {
            Result::Err(From::from(TypeMismatchError("DataAddress|EnvAddress", "UNKNOWN")))
        },
    }
}

/// !
fn set<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    let value = vm.data_stack_mut().pop()?;
    match *top {
        Value::DataAddress(adr) => {
            vm.data_buffer_mut().set(adr, value)?;
            Result::Ok(())
        },
        Value::EnvAddress(adr) => {
            let base = vm.return_stack().peek()?;
            vm.env_stack_mut().set(base.stack_address(), adr, value)?;
            Result::Ok(())
        },
        _ => {
            Result::Err(From::from(TypeMismatchError("DataAddress|EnvAddress", "UNKNOWN")))
        },
    }
}

/// body
fn body<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    let top = vm.data_stack_mut().pop()?;
    match *top {
        Value::CodeAddress(adr) => {
            let inst = vm.code_buffer().get(adr)?;
            match inst {
                Instruction::Push(val) => {
                    vm.data_stack_mut().push(val);
                },
                _ => {
                    vm.data_stack_mut().push(top);
                }
            }
        },
        _ => {
            vm.data_stack_mut().push(top);
        }
    }
    Result::Ok(())
}

/// constant
fn constant<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
where E: std::fmt::Debug
{
    util::call_with_name(vm, |v, name|{
        let here = v.code_buffer().here();
        let value = v.data_stack_mut().pop()?;
        v.word_dictionary_mut().reserve_word_def(name, Word::new(here));
        v.code_buffer_mut().push(compile_push_value(value));
        v.code_buffer_mut().push(compile_return());
        v.code_buffer_mut().push(compile_word_terminator());
        v.word_dictionary_mut().complate_word_def()?;
        Result::Ok(())
    })
}