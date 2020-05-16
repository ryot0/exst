//! 
//! 実行コンテキスト
//! 
//! 実行パラメータのパース
//! 

extern crate exst_core;
extern crate clap;

use exst_core::lang::vm::VmExecution;
use exst_core::lang::resource::Resources;
use exst_core::lang::resource::StdResources;
use super::exec::Executor;

use clap::App;
use clap::Arg;

/// 実行コンテキスト
pub struct Context {
    /// デバッグモード
    debug_mode: bool,
    /// 実行開始モジュール
    start_module: String,
    /// 実行開始モジュールへ渡す引数
    args: Vec<String>,
}
impl Context {

    /// コンストラクタ
    pub fn new() -> Self {
        Context {
            debug_mode: false,
            start_module: String::from("%STDIN"),
            args: Vec::new(),
        }
    }

    /// "name=value"の形式をnameとvalueに分割する
    /// 
    /// # Arguments
    /// * input - 解析対象の文字列
    /// 
    /// # Return Values
    /// (name, value)
    fn parse_variable(input: &str) -> (String,String) {
        let mut name = String::new();
        let mut value = String::new();
        let mut chars = input.chars();
        for c in &mut chars {
            if c == '=' {
                break;
            }
            name.push(c);
        }
        for c in &mut chars {
            value.push(c);
        }
        (name, value)
    }

    /// コマンド引数を解析する
    /// 
    /// # Arguments
    /// * app - アプリケーション情報が設定された状態のApp
    /// * itr - コマンド引数
    /// 
    /// # Panics
    /// 引数の解析失敗した場合は、エラー出力にヘルプを表示して異常終了する
    /// 
    /// # Return Values
    /// * 初期化されたリソース。これを使って後続処理でVmを初期化する
    pub fn parse_arg<I,T>(&mut self, app: App, itr: I) -> StdResources
        where I: IntoIterator<Item=T>, T: Into<std::ffi::OsString> + Clone
    {
        //引数の解析
        let matches = 
        app.arg(Arg::with_name("debug")
            .display_order(0)
            .help("debug mode flag")
            .short("d")
            .long("debug")
        ).arg(Arg::with_name("root")
            .display_order(1)
            .help("project root path")
            .short("r")
            .long("root")
            .default_value(".")
        ).arg(Arg::with_name("module")
            .display_order(2)
            .help("start module name")
            .short("m")
            .long("module")
            .default_value("%STDIN")
        ).arg(Arg::with_name("variables")
            .display_order(3)
            .help("extra resources")
            .short("v")
            .long("var")
            .takes_value(true)
            .multiple(true)
        ).arg(Arg::with_name("arguments")
            .display_order(3)
            .help("arguments")
            .short("a")
            .long("arg")
            .takes_value(true)
            .multiple(true)
        )
        .get_matches_from(itr);

        //プロパティの登録
        self.debug_mode = matches.is_present("debug");
        self.start_module = String::from(matches.value_of("module").unwrap());
        let project_root = String::from(matches.value_of("root").unwrap());

        //引数の解析
        let mut list = Vec::new();
        for args in matches.values_of("arguments") {
            for arg in args {
                list.push(String::from(arg));
            }
        }
        self.args = list;
        
        //リソースの作成
        let mut rs = StdResources::new(project_root);

        //リソースへ追加変数を登録
        for vars in matches.values_of("variables") {
            for var in vars {
                let (name, value) = Self::parse_variable(var);
                rs.add_resource(name, value);
            }
        }

        rs
    }

    /// 実行オブジェクト作成し、同時に自身の所有権を破棄
    /// 
    /// # Arguments
    /// * vm - VM
    /// 
    /// # Return Values
    /// 実行オブジェクト
    pub fn create<E,R,V>(self, vm: V) -> Executor<E,R,V>
        where R: Resources, V: VmExecution<ResourcesType=R,ExtraPrimitiveWordErrorReasonType=E>, E: std::fmt::Debug
    {
        Executor::new(vm, self)
    }

    /// デバッグモードかどうかを取得
    pub fn debug_mode(&self) -> bool {
        self.debug_mode
    }

    /// 開始モジュール名を取得
    pub fn start_module(&self) -> &String {
        &self.start_module
    }

    /// 開始モジュールに渡す引数を取得
    pub fn args(&self) -> &Vec<String> {
        &self.args
    }
}

#[cfg(test)]
mod tests {

    extern crate clap;
    extern crate exst_core;
    use super::Context;
    use clap::App;
    use exst_core::lang::resource::Resources;

    #[test]
    fn test_default() {

        let mut ctx = Context::new();
        let app = App::new("example");
        ctx.parse_arg(app, vec!["example"]);

        assert_eq!(ctx.debug_mode(), false);
        assert_eq!(ctx.start_module(), "%STDIN");
        assert_eq!(ctx.args().len(), 0);
    }

    #[test]
    fn test_short_debug() {
        
        let mut ctx = Context::new();
        let app = App::new("example");
        ctx.parse_arg(app, vec!["example", "-d"]);

        assert_eq!(ctx.debug_mode(), true);
        assert_eq!(ctx.start_module(), "%STDIN");
        assert_eq!(ctx.args().len(), 0);
    }

    #[test]
    fn test_long_debug() {
        
        let mut ctx = Context::new();
        let app = App::new("example");
        ctx.parse_arg(app, vec!["example", "--debug"]);

        assert_eq!(ctx.debug_mode(), true);
        assert_eq!(ctx.start_module(), "%STDIN");
        assert_eq!(ctx.args().len(), 0);
    }

    #[test]
    fn test_short_start_module() {
        
        let mut ctx = Context::new();
        let app = App::new("example");
        ctx.parse_arg(app, vec!["example", "-m", "xxx"]);

        assert_eq!(ctx.debug_mode(), false);
        assert_eq!(ctx.start_module(), "xxx");
        assert_eq!(ctx.args().len(), 0);
    }

    #[test]
    fn test_long_start_module() {
        
        let mut ctx = Context::new();
        let app = App::new("example");
        ctx.parse_arg(app, vec!["example", "--module", "xxx"]);

        assert_eq!(ctx.debug_mode(), false);
        assert_eq!(ctx.start_module(), "xxx");
        assert_eq!(ctx.args().len(), 0);
    }

    #[test]
    fn test_short_args() {
        
        let mut ctx = Context::new();
        let app = App::new("example");
        ctx.parse_arg(app, vec!["example", "-a", "A", "B", "C"]);

        assert_eq!(ctx.debug_mode(), false);
        assert_eq!(ctx.start_module(), "%STDIN");
        assert_eq!(ctx.args().len(), 3);
        assert_eq!(ctx.args()[0], "A");
        assert_eq!(ctx.args()[1], "B");
        assert_eq!(ctx.args()[2], "C");
    }

    #[test]
    fn test_long_args() {
        
        let mut ctx = Context::new();
        let app = App::new("example");
        ctx.parse_arg(app, vec!["example", "--arg", "A", "B", "C"]);

        assert_eq!(ctx.debug_mode(), false);
        assert_eq!(ctx.start_module(), "%STDIN");
        assert_eq!(ctx.args().len(), 3);
        assert_eq!(ctx.args()[0], "A");
        assert_eq!(ctx.args()[1], "B");
        assert_eq!(ctx.args()[2], "C");
    }

    #[test]
    fn test_short_vars() {
        
        let mut ctx = Context::new();
        let app = App::new("example");
        let res = ctx.parse_arg(app, vec!["example", "-v", "A=a", "B=", "C"]);

        assert_eq!(ctx.debug_mode(), false);
        assert_eq!(ctx.start_module(), "%STDIN");
        assert_eq!(res.get_string(&String::from("$A")).unwrap(), "a");
        assert_eq!(res.get_string(&String::from("$B")).unwrap(), "");
        assert_eq!(res.get_string(&String::from("$C")).unwrap(), "");
    }

    #[test]
    fn test_long_vars() {
        
        let mut ctx = Context::new();
        let app = App::new("example");
        let res = ctx.parse_arg(app, vec!["example", "--var", "A=a", "B=", "C"]);

        assert_eq!(ctx.debug_mode(), false);
        assert_eq!(ctx.start_module(), "%STDIN");
        assert_eq!(res.get_string(&String::from("$A")).unwrap(), "a");
        assert_eq!(res.get_string(&String::from("$B")).unwrap(), "");
        assert_eq!(res.get_string(&String::from("$C")).unwrap(), "");
    }
}