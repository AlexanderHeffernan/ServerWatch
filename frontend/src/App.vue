<template>
    <ConnectToServer v-if="!serverConnection" />
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
import ConnectToServer from './components/ConnectToServer.vue'
import { serverConnection } from './models/ServerConnection';
// import ServerConnection from './models/ServerConnection';

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
    --background-lighter-color: #546E7A;
    --background-dark-color: #181E20;
    --accent-light-color: #7fd982;
    --accent-color: #43A047;
    --accent-dark-color: #00701A;
    --text-color: #FFFFFF;
    --border-color: #4C5159;
}

* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
    font-family: 'Roboto', sans-serif;
}

#app {
    display: flex;
    flex-direction: row;
    width: 100vw;
    height: 100vh;
    padding: 0;
    margin: 0;
    background-color: #263238;
    position: absolute;
    top: 0;
    left: 0;
    overflow: hidden;
}

.main-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    flex-grow: 1;
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

p {
    color: #FFFFFF;
    font-size: 12px;
    font-weight: 400;
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