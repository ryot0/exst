//! 
//! システム関連ワード
//! 

use super::super::lang::vm::*;
use super::super::lang::resource::*;
use super::util;

/// システム関連ワードを登録
pub fn initialize<V>(vm: &mut V)
    where V: VmPrimitiveWordStore
{
    vm.define_primitive_word("exit".to_string(), false, exit);
    vm.define_primitive_word("bye".to_string(), false, bye);
    vm.define_primitive_word("[".to_string(), true, to_interpletation);
    vm.define_primitive_word("]".to_string(), false, to_compilation);
    vm.define_primitive_word("load".to_string(), false, load);
}

/// モジュールの終了
fn exit<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
{
    vm.set_state(VmState::Return);
    Result::Ok(())
}

/// プログラムの終了
fn bye<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
{
    vm.set_state(VmState::Stop);
    Result::Ok(())
}

/// コンパイルモードへ遷移
fn to_compilation<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
{
    vm.set_state(VmState::Compilation);
    Result::Ok(())
}

/// コンパイルモードへ遷移
fn to_interpletation<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
{
    vm.set_state(VmState::Interpretation);
    Result::Ok(())
}

/// モジュールのロード
fn load<V: VmManipulation,E>(vm: &mut V) -> Result<(),VmErrorReason<E>>
{
    util::call_with_name(vm, |v, name|{
        let target = v.resources().get_token_iterator(&name)?;
        v.call_script(target);
        Result::Ok(())
    })
}
