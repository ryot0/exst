//! 
//! コンパイル関連ワード
//! 

use super::super::lang::vm::*;

/// コンパイル関連ワードを登録
pub fn initialize<V>(_: &mut V)
    where V: VmPrimitiveWordStore
{
    //parse
    //parse_name
    //compile
    //[compile]
    //literal
    //postpone
    //特殊な命令にコンパイルされるワード
}

/// 起動時に実行するスクリプト
pub fn preload_script() -> &'static str
{r#"

"#}
