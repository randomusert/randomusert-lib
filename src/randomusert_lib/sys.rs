use fancy_sys_info::run;

pub mod sys {
    use fancy_sys_info::run;
    fn sys() {
        if let Err(err) = run() {
            eprintln!("Error: {}", err);
       }
    }
}
