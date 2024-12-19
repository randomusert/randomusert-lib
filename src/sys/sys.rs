use fancy_sys_info::run;

mod randomusert_lib {
    fn sys() {
        if let Err(err) = run() {
            eprintln!("Error: {}", err);
        }
    }
}

