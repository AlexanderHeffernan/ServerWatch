import { formatPercentage } from "../utils/formatters.js";

// Manage Chart.js gauges
const chartInstances = new Map();

export function drawGauge(value, ctx, canvasId) {
    if (chartInstances.has(canvasId)) {
        chartInstances.get(canvasId).destroy();
    }

    const newGauge = new Chart(ctx, {
        type: "doughnut",
        data: {
            datasets: [{
                data: [value, 100 - value],
                backgroundColor: ["#C51A4A", "rgba(0,0,0,0)"],
                borderWidth: 0,
                circumference: 270,
                rotation: 225,
                borderRadius: 10
            }]
        },
        options: {
            cutout: "80%",
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: { display: false },
                tooltip: { enabled: false }
            }
        },
        plugins: [{
            id: "centerText",
            afterDraw(chart) {
                const { ctx, width, height } = chart;
                ctx.save();
                ctx.font = "bold 20px Arial";
                ctx.fillStyle = "#C51A4A";
                ctx.textAlign = "center";
                ctx.textBaseline = "middle";
                ctx.fillText(formatPercentage(value), width / 2, height / 2);
                ctx.restore();
            }
        }]
    });

    chartInstances.set(canvasId, newGauge);
}