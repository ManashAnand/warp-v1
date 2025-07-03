use clap::builder::Str;
use sysinfo::{Components, Disks, Networks, System,Signal};

pub fn general_bool(status: bool, func_name: &str) {
    if status {
        if func_name == "mem" {
            show_mem();
        } else if func_name == "system" {
            show_system_specs();
        } else if func_name == "disk" {
            show_disk_specs();
        } else if func_name == "network" {
            show_network_specs();
        }
    }
}

fn show_mem() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("=> system:");
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());
    println!("");
}

fn print_info(label: &str, value: Option<String>) {
    match value {
        Some(val) => println!("{}: {:?}", label, val),
        None => println!("Error getting {}", label),
    }
}

fn show_system_specs() {
    let mut sys = System::new_all();

    sys.refresh_all();

    print_info("System name", System::name());
    print_info("Kernel version", System::kernel_version());
    print_info("OS version", System::os_version());
    print_info("Host name", System::host_name());

    println!("");
}

fn show_disk_specs() {
    let mut sys = System::new_all();

    sys.refresh_all();

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());

    println!("");
    // Display processes ID, name and disk usage:
    for (pid, process) in sys.processes() {
        println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
    }

    println!("");
    println!("=> disks:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("{disk:?}");
    }
    println!("");
}

fn show_network_specs() {
    let mut sys = System::new_all();

    sys.refresh_all();
    let networks = Networks::new_with_refreshed_list();
    println!("=> networks:");
    for (interface_name, data) in &networks {
        println!(
            "{interface_name}: {} B (down) / {} B (up)",
            data.total_received(),
            data.total_transmitted(),
        );
    }
    println!("");
}

pub fn kill_process_pid(pid: String) {
    println!("Kill this process {}", pid);

    let mut sys = System::new_all();
    sys.refresh_all();

    match sys.process(pid.parse().expect("converting string to PID")) {
        Some(process) => {
            println!("🔪 Killing process: {:?} [{}]", process.name(), pid);
            if process.kill_with(Signal::Kill).is_some() {
                println!("✅ Successfully sent kill signal.");
            } else {
                println!("⚠️ Failed to kill the process.");
            }
        }
        None => println!("❌ No process found with PID {}", pid),
    }
}
