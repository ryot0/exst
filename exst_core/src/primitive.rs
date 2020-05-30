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
use super::lang::tokenizer::*;
use super::lang::utility::*;

/// 文字列をロードし、関連ワードを定義する
/// 
/// # Panics
/// 組み込みのスクリプトにエラーがあった場合、パニックする
/// 
fn preload<V>(vm: &mut V, script: &'static str)
    where V: VmExecution
{
    let token =
        Box::new(
            TokenStream::new(
                CharStreamFromBufRead::new(
                    std::io::BufReader::new(
                        CharReaderFromString::new(String::from(script))
                    )
                )
            )
        );
    vm.call_script(token);
    vm.exec().unwrap();
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
    exception::initialize(vm);
    controlflow::initialize(vm);
    io::initialize(vm);
    debug::initialize(vm);

    // 関連するワードを組み込みスクリプトから登録
    preload(vm, stack::preload_script());
    preload(vm, data::preload_script());
    preload(vm, system::preload_script());
    preload(vm, compile::preload_script());
    preload(vm, word::preload_script());
    preload(vm, arithmetic::preload_script());
    preload(vm, exception::preload_script());
    preload(vm, controlflow::preload_script());
    preload(vm, io::preload_script());
    preload(vm, debug::preload_script());
}
