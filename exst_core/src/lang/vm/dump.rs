//! 
//! デバッグ用のVm状態のダンプ関数
//! 

use super::*;
use std::fmt;

/// format!マクロなどに渡すためのラッパー
/// 
/// Vmオブジェクトとそれを引数にとる関数を保持し、Display Traitの中で呼び出す
/// 
pub struct VmDump<'a, V: VmManipulation>( pub &'a V, pub fn(&V, &mut fmt::Formatter) -> fmt::Result );
impl<'a, V> fmt::Display for VmDump<'a, V>
    where V: VmManipulation
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.1(self.0, f)
    }
}
/// format!マクロなどに渡すためのラッパーの1引数版
/// 
/// Vmオブジェクトと引数１つとそれを引数にとる関数を保持し、Display Traitの中で呼び出す
/// 
pub struct VmDump1<'a, V: VmManipulation, T>( pub &'a V, &'a T, pub fn(&V, &T, &mut fmt::Formatter) -> fmt::Result );
impl<'a, V, T> fmt::Display for VmDump1<'a, V, T>
    where V: VmManipulation
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.2(self.0, self.1, f)
    }
}

/// VM状態（サマリ情報）の出力
/// 
/// # Arguments
/// * v - Vm
/// * f - 出力先
pub fn dump_vm_sate<V: VmManipulation>(v: &V, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "vm state: {}", v.state())?;
    writeln!(f, "program counter: {}", v.program_counter())?;
    writeln!(f, "script name: {}", 
        match v.debug_info_store().current_script_name() {
            Some(script_name) => {
                script_name
            },
            None => {
                ""
            },
        }
    )?;
    writeln!(f, "line number: {}", v.line_number())?;
    writeln!(f, "column number: {}", v.column_number())?;
    writeln!(f, "size of word dictionary: {}", v.word_dictionary().len())?;
    writeln!(f, "size of local dictionary: {}", v.local_dictionary().len())?;
    writeln!(f, "size of data stack: {}", v.data_stack().here())?;
    writeln!(f, "size of return stack: {}", v.return_stack().here())?;
    writeln!(f, "size of environment stack: {}", v.env_stack().here())?;
    writeln!(f, "size of control flow stack: {}", v.controlflow_stack().here())?;
    writeln!(f, "size of data buffer: {}", v.data_buffer().here())?;
    writeln!(f, "size of code buffer: {}", v.code_buffer().here())
}

/// 定義済みの全てのワード名を出力
/// 
/// # Arguments
/// * v - Vm
/// * f - 出力先
pub fn dump_all_word<V: VmManipulation>(v: &V, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", v.word_dictionary())
}

/// コードアドレスとスクリプトの対応を出力
/// 
/// # Arguments
/// * v - Vm
/// * adr - アドレス
/// * f - 出力先
fn dump_code_mapping<V: VmManipulation>(v: &V, adr: CodeAddress, f: &mut fmt::Formatter) -> fmt::Result {
    match adr {
        CodeAddress(Address::Entity(_)) => {
            match v.debug_info_store().get(adr) {
                Some(db) => {
                    match v.debug_info_store().get_script_name(db.script_name_handle) {
                        Some(script_name) => {
                            write!(f, "|{},{},{}|", script_name, db.line_number, db.column_number)
                        },
                        None => {
                            write!(f, "|-,{},{}|", db.line_number, db.column_number)
                        },
                    }
                },
                None => {
                    write!(f, "|-,-,-|")
                },
            }
        },
        CodeAddress(Address::Root) => {
            match v.debug_info_store().current_script_name() {
                Some(script_name) => {
                    write!(f, "|{},{},{}|", script_name, v.line_number(), v.column_number())
                },
                None => {
                    write!(f, "|-,{},{}|", v.line_number(), v.column_number())
                },
            }
        }
    }
}

/// コードの内容をダンプ
/// 
/// # Arguments
/// * adr - コードアドレス
/// * f - 出力先
fn dump_code<V: VmManipulation>(v: &V, adr: CodeAddress, f: &mut fmt::Formatter) -> fmt::Result {
    match adr {
        CodeAddress(Address::Entity(_)) => {
            match v.code_buffer().get(adr) {
                Result::Ok(inst) => {
                    write!(f, "  {}: {} ", adr, inst)?;
                    dump_code_mapping(v, adr, f)
                },
                Result::Err(_) => {
                    write!(f, "")
                },
            }
        },
        CodeAddress(Address::Root) => {
            write!(f, "  _: _ ")?;
            dump_code_mapping(v, adr, f)
        }
    }
}

/// 指定されたワードのコードを出力
/// 
/// # Arguments
/// * v - Vm
/// * f - 出力先
/// * name - ワード名
/// 
pub fn dump_word_code<V: VmManipulation>(v: &V, name: &String, f: &mut fmt::Formatter) -> fmt::Result {

    //ワードの取得とワード名の判定
    let word = v.word_dictionary().find_word(name);
    let (word_info, mut adr) = match word {
        Result::Ok(w) => (w.to_string(), w.code()),
        Result::Err(_) => (String::from("#UNDEFINED#"), CodeAddress::default())
    };

    writeln!(f, "{}: {}{{", name, word_info)?;
    
    //コード内容を出力
    while let Result::Ok(inst) = v.code_buffer().get(adr) {
        dump_code(v, adr, f)?;
        writeln!(f, "")?;
        match inst {
            Instruction::DebugLabel(DebugLabel::WordTerminator) => {
                //word terminatorまで出力したら、そのワードコードの終端とする
                break;
            },
            _ => {
                adr = adr.next();
            }
        }
    }

    writeln!(f, "}}")
}

/// すべてのワードのコードを出力
/// 
/// # Arguments
/// * v - Vm
/// * f - 出力先
pub fn dump_all_word_code<V: VmManipulation>(v: &V, f: &mut fmt::Formatter) -> fmt::Result {
    let names = v.word_dictionary().all_word_names();
    for n in names {
        dump_word_code(v, n, f)?;
    }
    Result::Ok(())
}

/// 環境スタックの内容をコールフレーム単位で出力
/// 
/// # Arguments
/// * base - 基準アドレス
/// * end_pos - 相対アドレスの終了アドレス
/// * f - 出力先
fn dump_env_stack<V: VmManipulation>(v: &V, base: EnvironmentStackAddress, end_pos: EnvironmentStackRelativeAddress, f: &mut fmt::Formatter) -> fmt::Result {
    let mut pos = EnvironmentStackRelativeAddress::default();
    while pos != end_pos {
        match v.env_stack().get(base, pos) {
            Result::Ok(v) => {
                writeln!(f, "    <{},{}>", pos, v)?;
            },
            Result::Err(_) => {
                break;
            }
        }
        pos = pos.next();
    }
    write!(f, "")
}

/// 環境スタック／リターンスタックの状態を出力
/// 
/// # Arguments
/// * v - Vm
/// * f - 出力先
pub fn dump_env<V: VmManipulation>(v: &V, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "ReturnStack[")?;
    let rs = v.return_stack();
    for i in 0 .. rs.here() {
        let current = rs.get(From::from(i)).unwrap();
        let next_adr = match rs.get(From::from(i + 1)) {
            Result::Ok(next_call_frame) => next_call_frame.stack_address().0,
            Result::Err(_) => v.env_stack().here().0,
        };
        write!(f, "  {}: ", current)?;
        dump_code(v, current.return_address(), f)?;
        writeln!(f, "")?;
        dump_env_stack(v, current.stack_address(), From::from(next_adr - current.stack_address().0), f)?;
    }
    writeln!(f, "]<-StackTop")
}

/// データスタックの内容を出力
/// 
/// # Arguments
/// * v - Vm
/// * f - 出力先
pub fn dump_data_stack<V: VmManipulation>(v: &V, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", v.data_stack())
}

/// コントロールフロースタックの内容を出力
/// 
/// # Arguments
/// * v - Vm
/// * f - 出力先
pub fn dump_controlflow_stack<V: VmManipulation>(v: &V, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", v.controlflow_stack())
}

/// すべての情報を出力
/// 
/// # Arguments
/// * v - Vm
/// * f - 出力先
pub fn dump_all_info<V: VmManipulation>(v: &V, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "##### Vm State #########################################")?;
    dump_vm_sate(v, f)?;
    writeln!(f, "##### Word Definition ##################################")?;
    dump_all_word_code(v, f)?;
    writeln!(f, "##### Controlflow Stack ################################")?;
    dump_controlflow_stack(v, f)?;
    writeln!(f, "##### Return Stack Environment #########################")?;
    dump_env(v, f)?;
    writeln!(f, "##### Data Stack #######################################")?;
    dump_data_stack(v, f)?;
    writeln!(f, "########################################################")
}