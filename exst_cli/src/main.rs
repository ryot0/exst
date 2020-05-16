
#[macro_use] 
extern crate clap;
extern crate exst_core;
extern crate exst_repl;

use clap::App;
use exst_core::lang::vm::*;
use exst_core::lang::resource::*;
use exst_core::primitive;
use exst_repl::context::Context;

type V = Vm<usize,usize,StdResources>;

fn main() {
    //clapの初期化
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!());
    //コマンドライン引数のパース
    let mut ctx = Context::new();
    let res = ctx.parse_arg(app, std::env::args());
    //vmの初期化
    let mut vm: V = Vm::new(res);
    primitive::initialize(&mut vm);
    //実行
    let e = ctx.create(vm);
    let return_code = e.exec();
    //終了処理
    std::process::exit(return_code);
}
