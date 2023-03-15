use std::env;

pub(crate) fn get_current_working_dir() -> String{
    let cwd = env::current_dir().unwrap();
    let my_str: String = cwd.as_os_str().to_str().unwrap().to_string();

    return my_str;
}
