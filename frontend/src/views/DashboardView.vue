<template>
    <div class="dashboard">
        <div class="widget wide" :class="{ moving: isMoving }">
            <div class="widget-header">
                <i class="fa-solid fa-microchip"></i>
                <h2>CPU Usage</h2>
            </div>
            <div class="widget-split">
                <div class="widget-section">
                    <div class="progress-bar">
                        <div class="progress" :style="{ width: `${totalCpuUsage}%` }"></div>
                    </div>
                    <p class="main-metric">Total CPU Usage: {{ totalCpuUsage }}%</p>
                    <br />
                    <p style="font-weight: 600;">Individual Core Usage:</p>
                    <p v-for="(usage, index) in individualCpuUsages" :key="index">
                        Core {{ index + 1 }}: {{ usage }}%
                    </p>
                </div>
                <div class="widget-section">
                    <div class="figure-metric">
                        <p class="figure"><i class="fa-solid fa-temperature-three-quarters"></i> {{ cpuTemperature }}°C</p>
                    </div>
                    <a style="margin-left: auto; margin-right: auto;">CPU Temperature Settings</a>
                </div>
            </div>
        </div>
        <div class="widget small" :class="{ moving: isMoving }">
            <div class="widget-header">
                <i class="fa-solid fa-memory"></i>
                <h2>RAM Usage</h2>
            </div>
            <div class="progress-bar">
                <div class="progress" :style="{ width: `${(memoryUsage / memoryTotal) * 100}%` }"></div>
            </div>
            <p class="main-metric">Used: {{ memoryUsage }}GB / Total: {{ memoryTotal }}GB</p>
        </div>
        <div class="widget small" :class="{ moving: isMoving }">

        </div>
    </div>
</template>

<script setup lang="ts">
import { inject, computed } from 'vue';
import { serverConnection } from '../models/ServerConnection';

const isMoving = inject('isSidebarMoving');

const totalCpuUsage = computed(() => {
    const usage = serverConnection.value?.getMetric('total_cpu_usage');
    return usage ? Math.round(Number(usage)) : 0;
});

const individualCpuUsages = computed(() => {
    const usages = serverConnection.value?.getMetric('individual_cpu_usage');

    return Array.isArray(usages) ? usages.map((usage: string) => Math.round(Number(usage))) : [];
});

const cpuTemperature = computed(() => {
    const temp = serverConnection.value?.getMetric('cpu_temperature');
    return temp ? Math.round(Number(temp)) : 0;
});

const memoryUsage = computed(() => {
    const usage = serverConnection.value?.getMetric('memory_usage');
    return usage ? convertBytesToGB(Number(usage)) : 0;
});
const memoryTotal = computed(() => {
    const total = serverConnection.value?.getMetric('memory_total');
    return total ? convertBytesToGB(Number(total)) : 0;
});

function convertBytesToGB(bytes: number): number {
    return Math.round((bytes / (1024 * 1024 * 1024)) * 100) / 100;
}
</script>

<style scoped>
.dashboard {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-start;
    align-items: flex-start;
    gap: 15px;
    width: 100%;
    height: 100%;
}

.widget {
    background-color: var(--background-light-color);
    border-radius: 13px;
    opacity: 1;
    transition: transform 0.3s ease, opacity 0.3s ease;
    padding: 15px;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    box-sizing: border-box;
}

.widget.moving {
    opacity: 0.3;
    transform: scale(0.95);
}

.widget.small {
    width: 250px;
    height: 200px;
}

.widget.wide {
    width: 500px;
    height: 200px;
}

@media (max-width: 768px) {
    .widget {
        width: 100% !important;
        height: auto !important;
        align-items: center;
    }

    .widget-header {
        width: 100%;
    }

    .widget-split {
        width: 100%;
        flex-direction: column;
        align-items: center;
    }

    .widget-split .widget-section {
        width: 100%;
        align-items: center;
    }
}

.widget-header {
    display: flex;
    align-items: center;
    gap: 10px;
    border-bottom: 2px solid var(--text-color);
    padding: 5px;
}

.widget-header i {
    font-size: 18px;
    color: var(--text-color);
}

.widget .progress-bar {
    background-color: var(--background-lighter-color);
    border-radius: 10px;
    width: 100%;
    height: 20px;
    margin-top: 10px;
    overflow: hidden;
}
.widget .progress {
    background-color: var(--primary-color);
    height: 100%;
    border-radius: 10px;
    transition: width 0.3s ease;
}

.widget .main-metric {
    font-size: 14px;
    color: var(--text-color);
    margin-top: 10px;
    font-weight: 600;
}

.widget-split {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 10px;
    height: 100%;
}

.widget-section {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: flex-start;
    height: 100%;
}

.figure-metric {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 100px;
    height: 100px;
    border-radius: 50%;
    background-color: var(--primary-color);
    margin: auto;
}

.figure-metric .figure {
    font-size: 24px;
    color: var(--text-color);
    font-weight: 600;
    margin: 0;
}

.figure-metric .label {
    font-size: 14px;
    color: var(--text-color);
    margin-top: 5px;
    text-align: center;
}

a {
    color: var(--text-color);
    text-decoration: underline;
    font-size: 12px;
    font-weight: 400;
    margin-top: 10px;
    cursor: pointer;
    transition: color 0.3s ease;
}

a:hover {
    color: var(--primary-color);
}
</style>