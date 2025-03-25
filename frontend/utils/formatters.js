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