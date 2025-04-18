<template>
    <SideNav @sidebar-toggled="handleSidebarToggle" />
    <div class="main-container">
        <TopBar @mobile-sidebar-toggle="handleMobileSidebarToggle" />
        <div class="content">
            <router-view />
        </div>
    </div>
</template>

<script lang="ts" setup>
import { ref, provide } from 'vue';
import SideNav from './components/SideNav.vue';
import TopBar from './components/TopBar.vue';

const isSidebarMoving = ref(false);
const mobileSidebarMinimized = ref(true);

function handleSidebarToggle() {
    if (window.innerWidth <= 480) {
        mobileSidebarMinimized.value = !mobileSidebarMinimized.value;
    }
    isSidebarMoving.value = true;
    setTimeout(() => {
        isSidebarMoving.value = false;
    }, 400);
}

function handleMobileSidebarToggle() {
    mobileSidebarMinimized.value = !mobileSidebarMinimized.value;
    isSidebarMoving.value = true;
    setTimeout(() => {
        isSidebarMoving.value = false;
    }, 400);
}

provide('isSidebarMoving', isSidebarMoving);
provide('mobileSidebarMinimized', mobileSidebarMinimized);

</script>

<style>
:root {
    --primary-color: #C51A4A;
    --primary-dark-color: #5D1A2A;
    --primary-light-color: #E0557A;
    --background-color: #263238;
    --background-light-color: #37474F;
    --accent-color: #43A047;
    --accent-dark-color: #00701A;
}

* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
    font-family: 'Roboto', sans-serif;
}

#app {
    display: flex;
    width: 100vw;
    height: 100vh;
    background-color: #263238;
    position: absolute;
    top: 0;
    left: 0;
    overflow: hidden;
}

.main-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
}

.content {
    width: 100%;
    margin: 15px;
}

@media (max-width: 480px) {
    .main-container {
        position: absolute;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
    }

    .content {
        margin: 0;
        padding: 15px;
        overflow-y: auto;
    }
}

h1 {
    color: #FFFFFF;
    font-size: 18px;
    font-weight: 800;
}

h2 {
    color: #FFFFFF;
    font-size: 14px;
    font-weight: 600;
}

@media (max-width: 480px) {
    h1 {
        font-size: 16px;
    }

    h2 {
        font-size: 12px;
    }
}
</style>