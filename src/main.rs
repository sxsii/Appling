use sysinfo::System;

fn main() {
    // Initialize the System struct and collect all info
    let mut sys = System::new_all();
    sys.refresh_all();

    // Get OS version
    let os = System::long_os_version().unwrap_or_else(|| "Unknown OS".to_string());

    // Get CPU brand
    let cpu_brand = sys.global_cpu_info().brand().to_string();
    let cpu_display = if cpu_brand.is_empty() { "Unknown CPU".to_string() } else { cpu_brand };

    // Total RAM in KB -> convert to GB
    let total_ram_kb = sys.total_memory();
    let total_ram_gb = total_ram_kb / 1024 / 1024;

    println!("Operating System: {}", os);
    println!("CPU: {}", cpu_display);
    println!("Total RAM: {} GB", total_ram_gb);
}
