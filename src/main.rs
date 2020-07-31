mod taskschd;
mod ole_utils;

use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;

use chrono::prelude::*;
use failure::Error;

use taskschd::TaskService;

fn main() {
    if let Err(e) = fallible_main() {
        eprintln!("{}", e);
    }
}

fn fallible_main() -> Result<(), Error> {
    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        if args[1] == "register" {
            return register()
        } else if args[1] == "run-from-task" {
            return run()
        }
    }
    Err(failure::err_msg("bad command line"))
}

fn run() -> Result<(), Error> {
    let now = Utc::now();
    let mut output = File::create("C:\\ProgramData\\s4u_test_output.txt")?;
    output.write(now.to_rfc3339_opts(SecondsFormat::Secs, true).as_bytes())?;

    Ok(())
}

fn register() -> Result<(), Error> {
    let _co = comedy::com::ComApartmentScope::init_mta()?;
    let mut svc = TaskService::connect_local()?;
    let mut folder = svc.get_root_folder()?;
    let mut def = svc.new_task_definition()?;
    let mut exec = def.add_exec_action()?;
    exec.put_Path(&env::current_exe()?)?;
    exec.put_Arguments(&vec!["run-from-task".into()])?;
    let two_mins_from_now = Utc::now() + chrono::Duration::minutes(2);
    let _trigger = def.add_time_trigger(&try_to_bstring!(OsString::from(two_mins_from_now.to_rfc3339_opts(SecondsFormat::Secs, true)))?)?;
    def.create(&mut folder, &try_to_bstring!(OsString::from("S4U test task"))?, None)?;

    Ok(())
}
