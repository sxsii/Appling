use sysinfo::{Disks, System};

fn main() {
    // Tab selector decides what to run
    show_computer_specifications();
    // or
    show_task_manager();
    // or
    show_priority_list();
}

// Section 1
fn show_computer_specifications() {
    get_os_info();
    get_processor_info();
    get_memory_info();
    get_storage_info();
    get_directx_version();
    get_graphics_info();
}

// Section 2
fn show_task_manager() {
    show_processes();
    show_performance();
    show_app_history();
    show_startup_apps();
}

// Section 3
fn show_priority_list() {
    // logic
}

fn get_os_info() {
     // Get OS version
    let os = System::long_os_version().unwrap_or_else(|| "Unknown OS".to_string());

    println!("OS: {}", os);
}

fn get_processor_info() {
   // Get CPU brand
    let _cpu = sys.global_cpu_usage();
    let cpu_brand = sys
        .cpus()
        .first()
        .map(|_cpu| _cpu.brand().to_string())
        .unwrap_or_default();
    let cpu_display = if cpu_brand.is_empty() { "Unknown CPU".to_string() } else { cpu_brand };
}

fn get_memory_info() {
    let total_ram = sys.total_memory();
    let (r_value, r_unit) = bytes_to_readable(total_ram);

    println!("Total RAM: {} {}", r_value, r_unit);
}

fn get_storage_info() {
    let disks = Disks::new_with_refreshed_list();

    let total_bytes: u64 = disks.list().iter().map(|d| d.total_space()).sum();
    let available_bytes: u64 = disks.list().iter().map(|d| d.available_space()).sum();

    let used_bytes = total_bytes - available_bytes;

    let (used_val, used_unit) = bytes_to_readable(used_bytes);
    let (s_value, s_unit) = bytes_to_readable(total_bytes);

    println!("Total Storage: {} {}", s_value, s_unit);
    println!("Used Storage: {} {}", used_val, used_unit);
}

fn get_directx_version() {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(directx) = hklm.open_subkey("SOFTWARE\\Microsoft\\DirectX") {
        if let Ok(version) = directx.get_value::<String, _>("Version") {
            let human_readable = match version.as_str() {
                "4.09.00.0904" => "DirectX 9.0c",
                "4.10.0000.0904" => "DirectX 10",
                "4.11.0000.0904" => "DirectX 11",
                "4.12.0000.0904" => "DirectX 12",
                _ => "Unknown / Newer DirectX",
            };
            println!("DirectX Version: {} ({})", human_readable, version);
            return;
        }
    }
    println!("DirectX Version: Unknown"); 
}

fn get_graphics_info() {
    
}