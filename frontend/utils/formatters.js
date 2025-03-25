// Formatting functions for consistent data presentation
export function formatPercentage(value) {
    return `${Math.round(value)}%`;
}

export function formatMemory(bytes) {
    return `${(bytes / (1024 ** 3)).toFixed(2)}GB`;
}

export function formatIndividualCpu(usageArray) {
    return usageArray.map((usage, i) => `Core ${i + 1}: ${formatPercentage(usage)}`).join(", <br />");
}

export function formatTemperature(temp) {
    return `${Math.round(temp)}°C`;
}

export function formatIndividualTemperature(tempArray) {
    return tempArray.map(({ label, temperature, max, critical }) => 
        `${label}: ${formatTemperature(temperature)} (Max: ${formatTemperature(max)}, Critical: ${formatTemperature(critical)})`
    ).join(", <br />");
}