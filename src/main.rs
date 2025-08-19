use sysinfo::{Disks, System};

fn main() {
    // Initialize the System struct and collect all info
    let mut sys = System::new_all();
    sys.refresh_all();

    // Get OS version
    let os = System::long_os_version().unwrap_or_else(|| "Unknown OS".to_string());

    // Get CPU brand
    let _cpu = sys.global_cpu_usage();
    let cpu_brand = sys
        .cpus()
        .first()
        .map(|_cpu| _cpu.brand().to_string())
        .unwrap_or_default();
    let cpu_display = if cpu_brand.is_empty() { "Unknown CPU".to_string() } else { cpu_brand };

    // Get storage
      /* let total_bytes: u64 = sys.disks()
        .iter()
        .map(|d| d.total_space())
        .sum();

    let (s_value, s_unit) = bytes_to_readable(total_bytes);
*/

println!("=> disks:");
let disks = Disks::new_with_refreshed_list();
for disk in &disks {
    println!("{disk:?}");
}
    // Total RAM in KB -> convert to GB
    let b_total = sys.total_memory();
    let (r_value, r_unit) = bytes_to_readable(b_total);


    println!("Operating System: {}", os);
    println!("CPU: {}", cpu_display);
    println!("Total RAM: {} {}", r_value, r_unit);
    //println!("Total Storage: {} {}", s_value, s_unit);
}

// function to see whether the memory is in TB or GB
fn bytes_to_readable(bytes: u64) -> (u64, &'static str) {
    let bytes_f = bytes as f64;
    let tb = bytes_f / 1024.0 / 1024.0 / 1024.0 / 1024.0;

    if tb >= 1.0 {
        // TB case
        let rounded = tb.round() as u64;
        (rounded, "TB")
    } else {
        // GB case
        let gb = bytes_f / 1024.0 / 1024.0 / 1024.0;
        let rounded = gb.round() as u64;
        (rounded, "GB")
    }
}