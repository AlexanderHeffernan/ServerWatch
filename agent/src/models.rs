use serde::Serialize;

#[derive(Serialize)]
pub struct TemperatureComponent {
    pub label: String,
    pub temperature: u64,
    pub max: u64,
    pub critical: u64,
}

#[derive(Serialize)]
pub struct Disk {
    pub label: String,
    pub total: u64,
    pub used: u64,
}

#[derive(Serialize)]
pub struct Metrics {
    pub total_cpu_usage: f32,
    pub individual_cpu_usage: Vec<f32>,
    pub memory_usage: u64,
    pub memory_total: u64,
    pub cpu_temperature: u64,
    pub individual_temperatures: Vec<TemperatureComponent>,
    pub disks: Vec<Disk>,
}