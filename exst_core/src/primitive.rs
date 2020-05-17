//! 
//! 組み込みワードの初期化
//! 

mod arithmetic;
mod io;
mod system;
mod debug;
mod word;
mod exception;
mod controlflow;
mod util;

use super::lang::vm::*;

/// 組み込みワードをvmに登録する
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    system::initialize(vm);
    word::initialize(vm);
    arithmetic::initialize(vm);
    exception::initialize(vm);
    controlflow::initialize(vm);
    io::initialize(vm);
    debug::initialize(vm);
}
