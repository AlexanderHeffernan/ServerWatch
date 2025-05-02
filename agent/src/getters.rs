use crate::models::{Disk, TemperatureComponent};
use sysinfo::{Disks, System, Components};

pub fn get_total_cpu_usage(sys: &System) -> f32 {
    sys.cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .sum::<f32>() / sys.cpus().len() as f32
}

pub fn get_individual_cpu_usage(sys: &System) -> Vec<f32> {
    sys.cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .collect()
}

pub fn get_temperature() -> u64 {
    let components = Components::new_with_refreshed_list();
    for component in &components {
        if component.label().contains("cpu") {
            return component.temperature().unwrap_or(0.0) as u64;
        }
    }
    if components.is_empty() {
        return 0;
    }
    components[0].temperature().unwrap_or(0.0) as u64
}

pub fn get_individual_temperatures() -> Vec<TemperatureComponent> {
    let components = Components::new_with_refreshed_list();
    components
        .iter()
        .map(|component| TemperatureComponent {
            label: component.label().to_string(),
            temperature: component.temperature().unwrap_or(0.0) as u64,
            max: component.max().unwrap_or(0.0) as u64,
            critical: component.critical().unwrap_or(0.0) as u64,
        })
        .collect()
}

pub fn get_disks() -> Vec<Disk> {
    Disks::new_with_refreshed_list()
        .iter()
        .map(|disk| Disk {
            label: disk.file_system().to_string_lossy().into_owned(),
            total: disk.total_space(),
            used: disk.total_space() - disk.available_space(),
        })
        .collect()
}

pub fn get_server_name() -> String {
    sysinfo::System::host_name().unwrap_or_else(|| "Unknown".to_string())
}