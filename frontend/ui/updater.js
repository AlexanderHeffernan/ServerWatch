import { drawGauge } from "../charts/gauges.js";
import { formatPercentage, formatMemory, formatIndividualCpu, formatTemperature, formatIndividualTemperature } from "../utils/formatters.js";

// Update UI elements with fetched data
export function updateUI(data) {
    // CPU Usage
    drawGauge(data.total_cpu_usage, document.getElementById("cpu-gauge").getContext("2d"), "cpu-gauge");
    document.getElementById("individualCpuUsage").innerHTML = formatIndividualCpu(data.individual_cpu_usage);

    // Memory Usage
    const memoryPercent = (data.memory_usage / data.memory_total) * 100;
    drawGauge(memoryPercent, document.getElementById("memory-gauge").getContext("2d"), "memory-gauge");
    document.getElementById("memoryUsage").textContent = formatMemory(data.memory_usage);
    document.getElementById("memoryTotal").textContent = formatMemory(data.memory_total);
    document.getElementById("memoryFree").textContent = formatMemory(data.memory_total - data.memory_usage);
    document.getElementById("memoryPercentage").textContent = formatPercentage(memoryPercent);

    // CPU Temperature
    document.getElementById("cpuTemperature").textContent = formatTemperature(data.cpu_temperature);
    document.getElementById("individualComponentTemp").innerHTML = formatIndividualTemperature(data.individual_temperatures)

    // Disk Usage
    drawGauge(data.disks[0].used / data.disks[0].total * 100, document.getElementById("disk-gauge").getContext("2d"), "disk-gauge");
    document.getElementById("individualDiskUsage").innerHTML = data.disks.map(disk => 
        `${disk.label}: Used: ${formatMemory(disk.used)}, Total: ${formatMemory(disk.total)}, Percentage: ${formatPercentage((disk.used / disk.total) * 100)}`
    ).join(", <br />");
}