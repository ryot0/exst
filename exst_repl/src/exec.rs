//! 
//! 実行処理のメイン
//! 

extern crate exst_core;

use exst_core::lang::vm::VmExecution;
use exst_core::lang::vm::VmErrorReason;
use exst_core::lang::resource::Resources;
use super::context::Context;

/// 実行
pub struct Executor<E,R,V>
    where R: Resources, V: VmExecution<ResourcesType=R,ExtraPrimitiveWordErrorReasonType=E>, E: std::fmt::Debug
{
    /// vm
    vm: V,
    /// 実行パラメータ
    ctx: Context,
}
impl<E,R,V> Executor<E,R,V>
    where R: Resources, V: VmExecution<ResourcesType=R,ExtraPrimitiveWordErrorReasonType=E>, E: std::fmt::Debug
{

    /// コンストラクタ
    /// 
    /// # Arguments
    /// * vm - VM
    /// * ctx - コンテキスト
    pub fn new(vm: V, ctx: Context) -> Self {
        Executor {
            vm,
            ctx,
        }
    }

    /// 実行
    /// 
    /// # Arguments
    /// * vm - VM
    /// * module - 開始モジュール
    /// * args - 引数
    /// 
    /// # Return Values
    /// 実行結果
    fn simple_exec(vm: &mut V, module: &String, args: &Vec<String>) -> Result<(),VmErrorReason<E>> {
        let start_module = vm.resources().get_token_iterator(module)?;
        vm.call_script(start_module);
        vm.reset_vm_state();
        vm.exec_with_args(args)
    }

    /// エラーの出力
    fn print_error(e: VmErrorReason<E>) {
        eprintln!("#### ERROR: {:?} ####", e);
    }

    /// デバッグモードの開始メッセージ
    fn print_debug_mode() {
        println!("#### START DEBUG ####");
    }

    /// 実行
    pub fn exec(mut self) -> i32 {
        let result = Self::simple_exec(&mut self.vm, self.ctx.start_module(), self.ctx.args());
        match result {
            Result::Err(e) => {
                Self::print_error(e);
                if self.ctx.debug_mode() {
                    Self::print_debug_mode();
                    while let Result::Err(e) = Self::simple_exec(&mut self.vm, &String::from("%STDIN"), &Vec::new()) {
                        Self::print_error(e);
                        Self::print_debug_mode();
                    }
                }
                100
            },
            Result::Ok(_) => {
                0
            }
        }
    }
}