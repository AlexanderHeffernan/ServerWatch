<template>
    <div class="dashboard">
        <div class="widget wide" :class="{ moving: isMoving }">

        </div>
        <div class="widget small" :class="{ moving: isMoving }">

        </div>
        <div class="widget small" :class="{ moving: isMoving }">

        </div>
    </div>
</template>

<script setup lang="ts">
import { inject } from 'vue';

const isMoving = inject('isSidebarMoving');
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
</style>