//! 
//! 組み込みワードの初期化
//! 

mod stack;
mod data;
mod arithmetic;
mod logical;
mod bit;
mod io;
mod system;
mod debug;
mod compile;
mod word;
mod exception;
mod controlflow;
mod util;

use super::lang::vm::*;
use super::lang::tokenizer::*;

/// 文字列をロードする
/// 
/// # Arguments
/// * script - ロードする文字列
/// 
/// # Return Values
/// ロードされたVm
/// 
pub fn preload<'a, V>(vm: &'a mut V, script: &'static str) -> &'a mut V
    where V: VmExecution
{
    let token = Box::new(new_token_stream_from_str(script));
    vm.call_script(token);
    vm
}

/// 組み込みワードをvmに登録する
/// 
/// # Panics
/// 組み込みのスクリプトにエラーがあった場合、パニックする
/// 
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore + VmExecution
{
    // 組み込みワードの登録
    stack::initialize(vm);
    data::initialize(vm);
    system::initialize(vm);
    compile::initialize(vm);
    word::initialize(vm);
    arithmetic::initialize(vm);
    logical::initialize(vm);
    bit::initialize(vm);
    exception::initialize(vm);
    controlflow::initialize(vm);
    io::initialize(vm);
    debug::initialize(vm);

    // 関連するワードを組み込みスクリプトから登録
    preload(vm, stack::preload_script()).exec().unwrap();
    preload(vm, data::preload_script()).exec().unwrap();
    preload(vm, system::preload_script()).exec().unwrap();
    preload(vm, compile::preload_script()).exec().unwrap();
    preload(vm, word::preload_script()).exec().unwrap();
    preload(vm, arithmetic::preload_script()).exec().unwrap();
    preload(vm, logical::preload_script()).exec().unwrap();
    preload(vm, bit::preload_script()).exec().unwrap();
    preload(vm, exception::preload_script()).exec().unwrap();
    preload(vm, controlflow::preload_script()).exec().unwrap();
    preload(vm, io::preload_script()).exec().unwrap();
    preload(vm, debug::preload_script()).exec().unwrap();
}
