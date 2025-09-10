use sysinfo::{Disks, /*Process, */System};
use winreg::{
    enums::HKEY_LOCAL_MACHINE,
    RegKey,
};
use windows::{
    Win32::Graphics::Dxgi::{
        CreateDXGIFactory1, IDXGIFactory1, DXGI_ADAPTER_DESC1, DXGI_ADAPTER_FLAG_SOFTWARE,
    }
};
use std::{thread, time::Duration};


fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Tab selector decides what to run
    show_computer_specifications(&sys);
    
    show_activity_monitor(&mut sys);
    
    //show_priority_list();
}

// for show_processes
#[derive(Debug)]
struct ProcessRow {
    pid: i32,
    name: String,
    cpu: f32,
    memory: String
}

// Section 1
fn show_computer_specifications(sys: &System) {
    get_os_info(sys);
    get_processor_info(sys);
    get_memory_info(sys);
    get_storage_info();
    get_directx_version();
    get_graphics_info();
}

// Section 2

fn show_activity_monitor(sys:&mut System) {
    show_processes(sys);
    //show_usage();
    //show_temperature();
}

/*
// Section 3
fn show_priority_list() {
    // logic
}
*/

fn get_os_info(_sys: &System) {
     // Get OS version
    let os = System::long_os_version().unwrap_or_else(|| "Unknown OS".to_string());

    println!("OS: {}", os);
}

fn get_processor_info(sys: &System) {
   // Get CPU brand
    let _cpu = sys.global_cpu_usage();
    let cpu_brand = sys
        .cpus()
        .first()
        .map(|_cpu| _cpu.brand().to_string())
        .unwrap_or_default();
    let _cpu_display = if cpu_brand.is_empty() { "Unknown CPU".to_string() } else { cpu_brand };
}

fn get_memory_info(sys: &System) {
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

    println!("Used Storage: {} {}", used_val, used_unit);
    println!("Total Storage: {} {}", s_value, s_unit);
}

fn show_processes(sys: &mut System) {
    for _ in 0..1 {
        let table = collect_process_table(sys);

        // Example: printing as a table in CLI
        println!("{:<8} {:<25} {:<10} {:<15}", "PID", "NAME", "CPU %", "MEM");
        for row in &table {
            println!("{:<8} {:<25} {:<10.2} {:<15}", row.pid, row.name, row.cpu, row.memory);
        }
        println!("--- Refreshing ---");

        thread::sleep(Duration::from_secs(2));
    }
}

// sub functions
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
    let _ = print_gpus();
}

fn print_gpus() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let factory: IDXGIFactory1 = CreateDXGIFactory1()?;

        let mut i = 0;
        loop {
            match factory.EnumAdapters1(i) {
                Ok(adapter) => {
                    let desc: DXGI_ADAPTER_DESC1 = adapter.GetDesc1()?;

                    let name = String::from_utf16_lossy(
                        &desc.Description
                            .iter()
                            .take_while(|&&c| c != 0)
                            .cloned()
                            .collect::<Vec<u16>>(),
                    );

                    // Skip software adapters or zero-VRAM dummies
                    if (desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32) != 0 || desc.DedicatedVideoMemory == 0 {
                        i += 1;
                        continue;
                    }

                    println!("GPU {}: {}", i, name);

                    let (vram_val, vram_unit) = bytes_to_readable(desc.DedicatedVideoMemory as u64);
                    println!("Dedicated VRAM: {} {}", vram_val, vram_unit);

                    let (shared_val, shared_unit) = bytes_to_readable(desc.SharedSystemMemory as u64);
                    println!("Shared System Memory: {} {}", shared_val, shared_unit);

                    i += 1;
                }
                Err(_) => break,
            }
        }

    }
    Ok(())
}

// function to see what unit of Bytes
fn bytes_to_readable(bytes: u64) -> (u64, &'static str) {
    let bytes_f = bytes as f64;

    let tb = bytes_f / 1024.0 / 1024.0 / 1024.0 / 1024.0;
    if tb >= 1.0 {
        return (tb.round() as u64, "TB");
    }

    let gb = bytes_f / 1024.0 / 1024.0 / 1024.0;
    if gb >= 1.0 {
        return (gb.round() as u64, "GB");
    }

    let mb = bytes_f / 1024.0 / 1024.0;
    if mb >= 1.0 {
        return (mb.round() as u64, "MB");
    }

    let kb = bytes_f / 1024.0;
    if kb >= 1.0 {
        return (kb.round() as u64, "KB");
    }

    // If it's less than 1 KB, return bytes
    (bytes, "B")
}

fn collect_process_table(sys: &mut System) -> Vec<ProcessRow> {
    sys.refresh_all();

    sys.processes()
        .iter()
        .map(|(pid, process)| {
            let (val, unit) = bytes_to_readable(process.memory());
            ProcessRow {
                pid: pid.as_u32() as i32,
                name: process.name().to_string_lossy().into_owned(),
                cpu: process.cpu_usage(),
                memory: format!("{} {}", val, unit),
            }
        })

        .collect()
}
