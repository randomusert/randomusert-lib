use fancy_sys_info::run;

pub mod sys;
fn sys() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
   }    
}


