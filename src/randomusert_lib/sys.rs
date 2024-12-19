use fancy_sys_info::run;

pub mod randomusert_lib {
    fn sys() {
        if let Err(err) = run() {
            eprintln!("Error: {}", err);
        }
    }
}

