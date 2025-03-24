let currentIP = "";
let currentPassword = "";
let demoMode = false;

const rememberMeCheck = document.getElementById("rememberMe");

if (localStorage.getItem("ipAddress") && localStorage.getItem("password")) {
    document.getElementById("ipAddress").value = localStorage.getItem("ipAddress");
    document.getElementById("password").value = localStorage.getItem("password");
    rememberMeCheck.checked = true;
}

async function fetchMetrics() {
    const ip = document.getElementById("ipAddress").value.trim();
    const password = document.getElementById("password").value.trim();

    if (!ip || !password) {
        alert("Please enter a valid IP address and password.");
        return;
    }

    currentIP = ip; // Save for refresh button
    currentPassword = password;
    try {
        await getMetrics(ip, password);
    } catch (error) {
        console.error("Error from fetching metrics!", error);
        return;
    }

    if (rememberMeCheck.checked) {
        localStorage.setItem("ipAddress", ip);
        localStorage.setItem("password", password);
    } else {
        localStorage.removeItem("ipAddress");
        localStorage.removeItem("password");
    }

    document.getElementById("connect-popup").classList.add("hidden");
}

async function refreshMetrics() {
    if ((currentIP && currentPassword) || demoMode) {
        try {		
            await getMetrics(currentIP, currentPassword);
        } catch (error) {
            console.error("Error from refreshing metrics!", error);
        }
    }
}

async function getMetrics(ip, password) {
    try {
        let data = {};
        console.log("Demo mode:",demoMode);
        if (!demoMode) {
            const response = await fetch(`http://${ip}:8080/metrics?password=${encodeURIComponent(password)}`);
        
            if (response.status === 401) {
                alert("Unauthorized: Incorrect password.");
                return new Error("Unable to fetch metrics: ", error);
            }

            data = await response.json();
        } else {
            data = {
                total_cpu_usage: Math.random() * 100,
                individual_cpu_usage: Array.from({ length: 4 }, () => Math.random() * 100),
                memory_usage: Math.random() * 1024 * 1024 * 1024 * 8,
                memory_total: 1024 * 1024 * 1024 * 16,
                disk_usage: Array.from({ length: 4 }, () => Math.random() * 1024 * 1024 * 1024 * 8)
            };
        }
        document.getElementById("totalCpuUsage").textContent = formatTotalCpuUsage(data.total_cpu_usage);
        document.getElementById("individualCpuUsage").innerHTML = formatIndividualCpuUsage(data.individual_cpu_usage);
        document.getElementById("memoryUsage").textContent = formatMemoryUsage(data.memory_usage);
        document.getElementById("memoryTotal").textContent = formatMemoryUsage(data.memory_total);
        document.getElementById("diskUsage").textContent = formatDiskUsage(data.disk_usage);
    } catch (error) {
        alert(`Unable to find agent at ${ip}:8080/metrics. Ensure agent script is running.`);
        throw new Error("Unable to fetch metrics!", error);
    }
}

function tryDemo() {
    demoMode = true;
    document.getElementById("connect-popup").classList.add("hidden");
    getMetrics();
}

function formatTotalCpuUsage(totalUsage) {
    return totalUsage.toFixed(0) + "%";
}

function formatIndividualCpuUsage(individualUsage) {
    return individualUsage.map((usage, index) => `Core ${index + 1}: ${usage.toFixed(0)}%`).join(", <br />");
}

function formatMemoryUsage(bytes) {
    return (bytes / 1024 / 1024 / 1024).toFixed(2) + "GB";
}

function formatDiskUsage(disk_usage) {
    // Disk usage is an array of the usage of each disk
    // Format disk usage from bytes used to GB
    return disk_usage.map((disk, index) => (disk / 1024 / 1024 / 1024).toFixed(2) + "GB").join(", ");
}