use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    let s = System::new_all();

    for process in s.processes_by_exact_name("vrmonitor.exe") {
        println!("{} {}", process.pid(), process.name());
    }

    for (pid, process) in s.processes() {
        println!("{} {}", pid, process.name());
    }
}
