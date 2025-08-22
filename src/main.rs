use sysinfo::{Disks, System};
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;
use windows::{
    Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIFactory1, IDXGIAdapter1},
    core::Interface,
};

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
    let disks = Disks::new_with_refreshed_list();

    let total_bytes: u64 = disks.list().iter().map(|d| d.total_space()).sum();
    let available_bytes: u64 = disks.list().iter().map(|d| d.available_space()).sum();

    let used_bytes = total_bytes - available_bytes;

    let (used_val, used_unit) = bytes_to_readable(used_bytes);
    let (s_value, s_unit) = bytes_to_readable(total_bytes);

    // Total RAM in KB -> convert to GB
    let total_ram = sys.total_memory();
    let (r_value, r_unit) = bytes_to_readable(total_ram);

    // DirectX version
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

    // Graphics
     unsafe {
        let mut factory: Option<IDXGIFactory1> = None;
        if CreateDXGIFactory1(&IDXGIFactory1::IID, &mut factory as *mut _ as *mut _).is_ok() {
            if let Some(factory) = factory {
                let mut i = 0;
                while let Ok(adapter) = factory.EnumAdapters1(i) {
                    let mut desc: DXGI_ADAPTER_DESC1 = std::mem::zeroed();
                    if adapter.GetDesc1(&mut desc).is_ok() {
                        let name = String::from_utf16_lossy(
                            &desc.Description
                                .iter()
                                .take_while(|&&c| c != 0)
                                .cloned()
                                .collect::<Vec<u16>>(),
                        );
                        println!("GPU {}: {}", i, name);
                        println!("Dedicated VRAM: {} MB", desc.DedicatedVideoMemory / 1024 / 1024);
                        println!("Shared System Memory: {} MB", desc.SharedSystemMemory / 1024 / 1024);
                    }
                    i += 1;
                }
            }
        }
    }

    // Printing
    println!("Operating System: {}", os);
    println!("CPU: {}", cpu_display);
    println!("Total RAM: {} {}", r_value, r_unit);
    println!("Storage Used:  {} {}", used_val, used_unit);
    println!("Total Storage: {} {}", s_value, s_unit);
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

