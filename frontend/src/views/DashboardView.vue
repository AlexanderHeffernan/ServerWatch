<template>
    <div class="dashboard">
        <div class="widget wide" :class="{ moving: isMoving }">
        </div>
        <div class="widget small" :class="{ moving: isMoving }">
            <div class="widget-header">
                <i class="fa-solid fa-memory"></i>
                <h2>RAM Usage</h2>
            </div>
            <div class="progress-bar">
                <div class="progress" :style="{ width: `${(memoryUsage / memoryTotal) * 100}%` }"></div>
            </div>
            <p class="main-metric">Used: {{ memoryUsage }}GB / Total: {{ memoryTotal }}</p>
        </div>
        <div class="widget small" :class="{ moving: isMoving }">

        </div>
    </div>
</template>

<script setup lang="ts">
import { inject, computed } from 'vue';
import { serverConnection } from '../models/ServerConnection';

const isMoving = inject('isSidebarMoving');

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
        height: auto;
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
    height: 20px;
    margin-top: 10px;
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
</style>