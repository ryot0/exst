pub mod lang;
pub mod primitive;

#[cfg(test)]
mod tests {

    use super::lang::vm::*;
    use super::lang::vm::dump;
    use super::lang::resource::*;
    use super::primitive::*;

    type V = Vm<usize,usize,StdResources>;

    fn exec_test(script: &'static str) {
        let res = StdResources::new(String::from("."));
        let mut vm: V = V::new(res);
        initialize(&mut vm);
        let result = preload(&mut vm, script).exec();
        match result {
            Result::Ok(_) => {
                if vm.data_stack().here() != From::from(0) {
                    print!("{}", dump::VmDump(&vm, dump::dump_all_info));
                    panic!("data stack is not empty.")
                }
            },
            Result::Err(e) => {
                print!("{}", dump::VmDump(&vm, dump::dump_all_info));
                panic!(e);
            }
        }
    }

    #[test]
    fn test() {
        exec_test(r#"
            1 2 + . cr
        "#);
    }
}