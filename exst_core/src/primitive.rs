//! 
//! 組み込みワードの初期化
//! 

mod stack;
mod data;
mod arithmetic;
mod io;
mod system;
mod debug;
mod compile;
mod word;
mod exception;
mod controlflow;
mod util;

use super::lang::vm::*;

/// 組み込みワードをvmに登録する
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    stack::initialize(vm);
    data::initialize(vm);
    system::initialize(vm);
    compile::initialize(vm);
    word::initialize(vm);
    arithmetic::initialize(vm);
    exception::initialize(vm);
    controlflow::initialize(vm);
    io::initialize(vm);
    debug::initialize(vm);
}
