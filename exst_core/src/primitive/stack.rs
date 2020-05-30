//! 
//! スタック操作関連ワード
//! 

use super::super::lang::vm::*;

/// スタック操作関連ワードを登録
pub fn initialize<V>(_: &mut V)
    where V: VmPrimitiveWordStore
{
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"

"#}