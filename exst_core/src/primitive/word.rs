//! 
//! ワード定義関連ワード
//! 

use super::super::lang::vm::*;

/// ワード定義関連ワードを登録
pub fn initialize<V>(_: &mut V)
    where V: VmPrimitiveWordStore
{
    //:
    //:noname
    //:recurse
    //;
    //immidiate
    //defer
    //'
    //[']
    //is
    //execute
    //create
    //does>
    //recurse
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"

"#}