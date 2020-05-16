//!
//! 実行を制御する
//! 
//! # Command Line Args
//! * -d (--debug) - デバッグモード
//! * -m (--module) [開始モジュール名] - 最初に実行するモジュール。省略した場合は標準入力から実行する。
//! * -r (--root) [プロジェクトルートパス] - モジュール参照時のルーとパス。省略した場合はカレントディレクトリ。
//! * -v (--var) [モジュール名]=[プログラム] ... - 追加内部モジュール
//! * -a (--arg) [モジュールへ渡す引数] ... - モジュールへ渡す引数
//! 
//! モジュールへ渡す引数は、環境スタックへ積まれる。左側の値からスタックに積んでいくためスタックトップは最後の引数となる。
//! -v引数で渡されたモジュールはプログラム内からは$[モジュール名]という名前のモジュールとして参照できる。
//! デバッグモードで起動した場合、異常終了した時にデバッグモードに遷移し標準入力からのプログラムをインタープリターモードで実行する。
//! 
//! # Examples
//! ```
//! extern crate exst_core;
//! extern crate clap;
//! 
//! use exst_repl::context::*;
//! use std::rc::Rc;
//! use clap::App;
//! use exst_core::lang::vm::*;
//! use exst_core::lang::resource::*;
//! use exst_core::lang::value::*;
//! 
//! type V = Vm<i32,i32,StdResources>;
//! type VError = VmErrorReason<i32>;
//! 
//! //２項目の足し算
//! fn plus(v: &mut V) -> Result<(),VError> {
//!     if let Value::IntValue(v1) = *v.data_stack_mut().pop().unwrap() {
//!         if let Value::IntValue(v2) = *v.data_stack_mut().pop().unwrap() {
//!             v.data_stack_mut().push(Rc::new(Value::IntValue(v1 + v2)));
//!         }
//!     }
//!     Result::Ok(())
//! }
//! 
//! //clapの初期化
//! let app = App::new("example");
//! //コマンドライン引数のパース
//! let mut ctx = Context::new();
//! let res = ctx.parse_arg(app, vec!["example", "-m", "$MAIN", "-v", "MAIN=1 2 +", "-a", "1", "2"]);
//! //vmの初期化
//! let mut vm: V = Vm::new(res);
//! vm.define_primitive_word(String::from("+"), false, plus);
//! //実行
//! let e = ctx.create(vm);
//! assert_eq!(e.exec(), 0);
//! ```
//! 
//! 

pub mod context;
pub mod exec;

#[cfg(test)]
mod tests {

    extern crate exst_core;
    extern crate clap;

    use super::context::*;
    use std::rc::Rc;
    use clap::App;
    use exst_core::lang::vm::*;
    use exst_core::lang::resource::*;
    use exst_core::lang::value::*;

    type V = Vm<i32,i32,StdResources>;
    type VError = VmErrorReason<i32>;

    //２項目の足し算
    fn plus(v: &mut V) -> Result<(),VError> {
        if let Value::IntValue(v1) = *v.data_stack_mut().pop().unwrap() {
            if let Value::IntValue(v2) = *v.data_stack_mut().pop().unwrap() {
                v.data_stack_mut().push(Rc::new(Value::IntValue(v1 + v2)));
            }
        }
        Result::Ok(())
    }

    #[test]
    fn test_main() {
        //clapの初期化
        let app = App::new("example");
        //コマンドライン引数のパース
        let mut ctx = Context::new();
        let res = ctx.parse_arg(app, vec!["example", "-m", "$MAIN", "-v", "MAIN=1 2 +", "-a", "1", "2"]);
        //vmの初期化
        let mut vm: V = Vm::new(res);
        vm.define_primitive_word(String::from("+"), false, plus);
        //実行
        let e = ctx.create(vm);
        assert_eq!(e.exec(), 0);
    }
}
