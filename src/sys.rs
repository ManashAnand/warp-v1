use sysinfo::{Components, Disks, Networks, System};

pub fn general_bool(status: bool, func_name: &str) {
    if status {
        if func_name == "mem" {
            show_mem();
        } 
        else if func_name == "system" {
            show_system_specs();
        }
    }
}

fn show_mem() {
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());
    println!("");
}

fn show_system_specs() {
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // Display system information:
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());
    println!("");
}
