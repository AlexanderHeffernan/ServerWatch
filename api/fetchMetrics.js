// Centralized API fetch logic with detailed error handling
export async function fetchMetrics(ip, password, demoMode = false) {
    if (demoMode) return generateDemoData();

    try {
        const response = await fetch(`https://${ip}:49160/metrics?password=${encodeURIComponent(password)}`, {
            timeout: 5000 // Prevents hanging
        });

        if (!response.ok) {
            if (response.status === 401) throw new Error("Unauthorized: Incorrect password.");
            if (response.status === 404) throw new Error(`Agent not found at ${ip}:49160/metrics. Ensure agent script is running.`);
            throw new Error(`HTTP error: ${response.status} - ${response.statusText}`);
        }
        return await response.json();
    } catch (error) {
        if (error.name === "TypeError") {
            // Check if it's likely a certificate error
            if (error.message.includes("certificate") && error.message.includes("invalid")) {
                throw {
                    message: "Certificate error: The server's self-signed certificate needs to be accepted.",
                    isCertError: true,
                    ip: ip
                }
            }
        }
        if (error.name === "TypeError") throw new Error(`Network error: Unable to connect to ${ip}:49160. Check network or agent status.`);
        throw error;
    }
}

function generateDemoData() {
    return {
        total_cpu_usage: Math.random() * 100,
        individual_cpu_usage: Array.from({ length: 4 }, () => Math.random() * 100),
        memory_usage: Math.random() * 1024 * 1024 * 1024 * 16,
        memory_total: 1024 * 1024 * 1024 * 32,
        cpu_temperature: Math.random() * 100,
        individual_temperatures: Array(4).fill().map((_, i) => ({
            label: String.fromCharCode(65 + i), // A, B, C, D
            temperature: Math.random() * 100,
            max: Math.random() * 100,
            critical: Math.random * 100
        })),
        disks: Array(4).fill().map((_, i) => ({
            label: String.fromCharCode(67 + i), // C, D, E, F
            total: Math.random() * 1024 * 1024 * 1024 * 8,
            used: Math.random() * 1024 * 1024 * 1024 * 8
        }))
    };
}