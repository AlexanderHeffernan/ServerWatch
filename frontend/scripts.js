let currentIP = "";
let currentPassword = "";
let demoMode = false;
const chartInstances = new Map(); // To store Chart.js instances by canvas ID

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
    
        drawGauge(data.total_cpu_usage, document.getElementById("cpu-gauge").getContext("2d"), "cpu-gauge");
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

// Function to flip the card
function flipCard(cardId) {
    const card = document.querySelector(`#${cardId} .card-flip`);
    card.classList.toggle('flipped');
}

function drawGauge(cpuUsage, ctx, canvasId) {
    // Destroy the previous chart instance for this canvas, if it exists
    if (chartInstances.has(canvasId)) {
        chartInstances.get(canvasId).destroy();
    }

    // Chart.js configuration for a circular gauge
    const newGauge = new Chart(ctx, {
        type: 'doughnut',
        data: {
            datasets: [{
                data: [cpuUsage, 100 - cpuUsage],
                backgroundColor: ['#0072F5', 'rgba(0,0,0,0)'],
                borderWidth: 0,
                circumference: 270,
                rotation: 225,
                borderRadius: 10
            }]
        },
        options: {
            cutout: '80%', // Thickness of the gauge
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: { display: false }, // Hide legend
                tooltip: { enabled: false } // Disable tooltips
            }
        },
        plugins: [{
            id: 'centerText',
            afterDraw(chart) {
                const { ctx, width, height } = chart;
                ctx.save();
                ctx.font = 'bold 20px Arial';
                ctx.fillStyle = '#0072F5';
                ctx.textAlign = 'center';
                ctx.textBaseline = 'middle';
                ctx.fillText(`${Math.round(cpuUsage)}%`, width / 2, height / 2);
                ctx.restore();
            }
        }]
    });

    // Store the new chart instance in the Map
    chartInstances.set(canvasId, newGauge);
}