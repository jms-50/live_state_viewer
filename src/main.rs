use sysinfo::{
    Components, Disks, Networks, System, Users, get_current_pid,
};
// use hostname;
use std::{
    process::Command,
};


fn format_bytes(bytes:u64) -> String{
    const UNITS:[&str; 5] = ["B","KiB","MiB","GiB","TiB"]; 
    if bytes == 0 {
        return "0 B".to_string();
    }
    let mut count = bytes as f64;
    let mut unit_index = 0;

    // divide 1024 loop 
    while count >= 1024.0 && unit_index < UNITS.len() - 1 {
        count /= 1024.0;
        unit_index += 1;
    }

    // print floating point .2 ({:.2})
    format!("{:.2} {}", count, UNITS[unit_index])
}

fn main() {
    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!

    if sysinfo::IS_SUPPORTED_SYSTEM {
        let mut sys = System::new_all();
        
        // First we update all information of our `System` struct.
        sys.refresh_all();

        // Users
        let users = Users::new_with_refreshed_list();
        

        println!("==================================================");
        // println!("host name = {:?}",host_name);
        
        // RAM and swap information:
        println!("=> RAM, swap info:");
        println!("total memory:            {}", format_bytes(sys.total_memory()));
        println!("used memory :            {}", format_bytes(sys.used_memory()));
        println!("total swap  :            {}", format_bytes(sys.total_swap()));
        println!("used swap   :            {}", format_bytes(sys.used_swap()));
        println!();
        // Display system information:
        println!("=> system info:");
        println!("System name:             {:?}", System::name().unwrap_or_else(|| "<unknown>".to_owned()));
        println!("System kernel version:   {:?}", System::kernel_version().unwrap_or_else(|| "<unknown>".to_owned()));
        println!("System OS version        {:?}", System::long_os_version().unwrap_or_else(|| "<unknown>".to_owned()));
        println!("System host name:        {:?}", System::host_name().unwrap_or_else(|| "<unknown>".to_owned()));
        println!();

        // Display proceesed user informaiton
        println!("=> user info:");
        if let Ok(current_pid) = get_current_pid() {
            if let Some(process) = sys.process(current_pid) {
                if let Some(user_id) = process.user_id() {
                    if let Some(user )= users.get_user_by_id(user_id) {
                        println!("User name:               {:?}", user.name());
                        println!("UID:                     {:?}", user.id());
                        println!("GID:                     {:?}", user.group_id());
                    }
                }
            }
        }
        println!();

        println!("=> disks:");
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            println!("disk name                {:?}", disk.name());
            println!("{:?} total space         {:?}", disk.name(),disk.total_space());
            println!("{:?} available space     {:?}", disk.name(),disk.available_space());
            println!();
        }

        // Number of CPUs:
        println!("NB CPUs:                 {}", sys.cpus().len());
        
        println!("UPtime:                  {}", System::uptime());

        println!("==================================================");
    }
    else{
        println!("This OS isn't supported.")
    }
    
    // // Display processes ID, name and disk usage:
    // for (pid, process) in sys.processes() {
    //     println!("[{pid}] {:?} {:?}", process.name(), process.disk_usage());
    // }

    // We display all disks' information:
    

    // // Network interfaces name, total data received and total data transmitted:
    // let networks = Networks::new_with_refreshed_list();
    // println!("=> networks:");
    // for (interface_name, data) in &networks {
    //     println!(
    //         "{interface_name}: {} B (down) / {} B (up)",
    //         data.total_received(),
    //         data.total_transmitted(),
    //     );
    //     // If you want the amount of data received/transmitted since last call
    //     // to `Networks::refresh`, use `received`/`transmitted`.
    // }

    // // Components temperature:
    // let components = Components::new_with_refreshed_list();
    // println!("=> components:");
    // for component in &components {
    //     println!("{component:?}");
    // }
}