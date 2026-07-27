use std::{
    fs,
    path::PathBuf
};
use sysinfo::{
    Disks, System, Users
};
use clap::{
    Parser, ValueEnum, ArgGroup
};
use serde::{
    Serialize, Deserialize
};
use directories::ProjectDirs;

// const OPTION:[&str; 3] = ["uptime","cpu","disk"];
// const ALL_FEATURES:&[&str; 6] = &["user","system","memory","disk","cpu","uptime"];

#[derive(clap::Subcommand, Debug)]
enum Modes {
    Btop,
    Scan,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
enum Features {
    User,
    System,
    Memory,
    Disk,
    Cpu,
    Uptime,
}

// Define the command-line arguments structure
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(group(ArgGroup::new("action").required(false).args(["on", "off", "reset",])))]
struct Args {
    #[arg(long, value_enum, value_delimiter = ',', num_args = 1..)]
    on: Option<Vec<Features>>,

    #[arg(long, value_enum, value_delimiter = ',', num_args = 1..)]
    off: Option<Vec<Features>>,

    #[arg(short, long, help = "Enable or disable system information gathering")]
    list: bool,
    
    #[arg(short, long, conflicts_with = "on",help = "Reset the configuration to default settings(system, memory, disk, uptime)")]
    reset: bool,

    #[command(subcommand)] 
    mode: Option<Modes>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    user: bool,
    system: bool,
    memory: bool,
    disk: bool,
    cpu: bool,
    uptime: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            user: false,
            system: true,
            memory: true,
            disk: true,
            cpu: false,
            uptime: true,
        }
    }
}

impl Config {
    // get the path to the configuration file
    fn config_path() -> PathBuf { 
        // different for each OS, using the directories crate
        // get the project directories for the application
        let proj_dirs = ProjectDirs::from("com", "msJms", "LiveStateViewer")
            .expect("config directory not found.");
        
        // get the configuration directory path
        let config_dir = proj_dirs.config_dir();
        
        // directory is not exist, create it
        if !config_dir.exists() {
            fs::create_dir_all(config_dir).expect("config directory creation failed");
        }
        
        config_dir.join("config.toml")
    }

    /// load the configuration from the file, or return default if not exist
    fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            let content = fs::read_to_string(path).unwrap_or_default();
            toml::from_str(&content).unwrap_or_else(|_| Config::default())
        } else {
            let default_config = Config::default();
            default_config.save();
            default_config
        }
    }

    /// save the configuration to the file
    fn save(&self) {
        let path = Self::config_path();
        let toml_string = toml::to_string(self).expect("config to toml serialization failed");
        fs::write(path, toml_string).expect("config file write failed");
    }

    fn enable(&mut self, feature: Features) {
        match feature {
            Features::User => self.user = true,
            Features::System => self.system = true,
            Features::Memory => self.memory = true,
            Features::Disk => self.disk = true,
            Features::Cpu => self.cpu = true,
            Features::Uptime => self.uptime = true,
        }
    }

    fn disable(&mut self, feature: Features) {
        match feature {
            Features::User => self.user = false,
            Features::System => self.system = false,
            Features::Memory => self.memory = false,
            Features::Disk => self.disk = false,
            Features::Cpu => self.cpu = false,
            Features::Uptime => self.uptime = false,
        }
    }

    fn is_enabled(&self, feature: Features) -> bool {
        match feature {
            Features::User => self.user,
            Features::System => self.system,
            Features::Memory => self.memory,
            Features::Disk => self.disk,
            Features::Cpu => self.cpu,
            Features::Uptime => self.uptime,
        }
    }
}

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
    // if the system is supported, we can proceed with gathering and displaying system information
    // if the system is not supported, we will print a message indicating that the OS is not supported
    if sysinfo::IS_SUPPORTED_SYSTEM {
        let args = Args::parse();
        match args.mode {
            Some(Modes::Btop) => {
                println!("System information gathering is enabled.");
            }
            Some(Modes::Scan) => {
                println!("System information gathering is disabled.");
            }
            None => {
                let mut config;
                if args.reset {
                    config = Config::default();
                    config.save();
                    println!("Configuration reset to default settings.");
                    if args.list {
                        for feature in Features::value_variants() {
                            let status = config.is_enabled(*feature);
                            println!("{:?}: {}",feature,status);
                        }
                    } 
                }
                else {
                    config = Config::load();
                    if let Some(features) = args.on {
                        for feature in features {
                            config.enable(feature);
                        }
                        config.save();
                        if args.list {
                        for feature in Features::value_variants() {
                            let status = config.is_enabled(*feature);
                            println!("{:?}: {}",feature,status);
                        }
                    }
                    }
                    else if let Some(features) = args.off {
                        for feature in features {
                            config.disable(feature);
                        } 
                        config.save();
                        if args.list {
                        for feature in Features::value_variants() {
                            let status = config.is_enabled(*feature);
                            println!("{:?}: {}",feature,status);
                        }
                    }
                    }
                    else if args.list {
                        for feature in Features::value_variants() {
                            let status = config.is_enabled(*feature);
                            println!("{:?}: {}",feature,status);
                        }
                    } else {
                        let mut sys = System::new_all(); // info initialization
                        sys.refresh_all(); // Update all information of System struct.
                        println!("==================================================");
                        // Users information:
                        // Display proceesed user informaiton
                        if config.is_enabled(Features::User) {
                            let users = Users::new_with_refreshed_list();
                            println!("=> user info:");
                            if let Ok(current_pid) = sysinfo::get_current_pid() {
                                if let Some(process) = sys.process(current_pid) {
                                    if let Some(user_id) = process.user_id() {
                                        if let Some(user )= users.get_user_by_id(user_id) {
                                            let user_name = user.name();
                                            let user_id = user.id();
                                            let user_group_id = user.group_id();
                                            println!("User name:               {:?}", user_name);
                                            println!("UID:                     {:?}", user_id);
                                            println!("GID:                     {:?}", user_group_id);
                                        }
                                    }
                                }
                            }
                            println!();
                        }
                        // System information:
                        // Display System info Object
                        if config.is_enabled(Features::System) {
                            let host_name = System::host_name().unwrap_or_else(|| "<unknown>".to_owned());
                            let os_version = System::long_os_version().unwrap_or_else(|| "<unknown>".to_owned());
                            let kernel_version = System::kernel_version().unwrap_or_else(|| "<unknown>".to_owned());
                            let kernel_name = System::name().unwrap_or_else(|| "<unknown>".to_owned());
                            println!("=> system info:");
                            println!("System host name:    :   {:?}", host_name.split('.').next().unwrap_or_else(|| "<unknown>"));
                            println!("System OS version    :   {:?}", os_version);
                            println!("System kernel name   :   {:?}", kernel_name);
                            println!("System kernel version:   {:?}", kernel_version);
                            println!();
                        }
                        // RAM and swap information:
                        // Display RAM and swap info Object    
                        if config.is_enabled(Features::Memory) {
                            let total_memory: u64 = sys.total_memory();
                            let used_memory: u64 = sys.used_memory(); 
                            let total_swap: u64 = sys.total_swap();
                            let used_swap: u64 = sys.used_swap();
                            // RAM and swap info Display
                            println!("=> Memory(RAM, SWAP) info:");
                            println!("Total Memory:            {}", format_bytes(total_memory));
                            println!("Used Memory :            {}", format_bytes(used_memory));
                            println!("Total SWAP  :            {}", format_bytes(total_swap));
                            println!("Used SWAP   :            {}", format_bytes(used_swap));
                            println!();
                        }
                        // Disk information:
                        // Display Disk info Object
                        if config.is_enabled(Features::Disk) {
                            println!("=> Disk info:");
                            let disks = Disks::new_with_refreshed_list();
                            let mut seen_disks = Vec::new();
                            for disk in &disks {
                                let disk_key = (disk.name().to_os_string(), disk.total_space());
                                if !seen_disks.contains(&disk_key) {
                                    seen_disks.push(disk_key);
                                } else {
                                    continue
                                }
                                println!("Disk name      :         {:?}", disk.name());
                                println!("Total SPACE    :         {:?}", format_bytes(disk.total_space()));
                                println!("Using SPCAE    :         {:?}", format_bytes(disk.total_space() - disk.available_space()));
                                println!("Available SPACE:         {:?}", format_bytes(disk.available_space()));
                                println!();
                            }
                        }
                        // CPU information:
                        // Display CPU info Object
                        if config.is_enabled(Features::Cpu) {
                            println!("=> CPU info:");
                            let cpus = sys.cpus();
                            for (i, cpu) in cpus.iter().enumerate() {
                                println!("CPU {}: {} MHz", i, cpu.frequency());
                            }
                            println!("NB CPUs:                 {}", cpus.len());
                            println!();
                        }
                        // Uptime information:
                        if config.is_enabled(Features::Uptime) {
                            println!("UPtime:                  {}", System::uptime());
                        }
                        println!("==================================================");
                        // println!("if you want see {:?},\nLiveStateViewer --on {:?}",ALL_FEATURES, ALL_FEATURES)
                    }
                }
            }
        }
        
    } 
    else{
        println!("This OS isn't supported.")
    }
}