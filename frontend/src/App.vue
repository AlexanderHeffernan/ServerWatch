<template>
    <ReconnectDialog v-if="savedIp && savedPassword && !serverConnection" />
    <ConnectToServer v-else-if="!serverConnection" />
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
import ReconnectDialog from './components/ReconnectDialog.vue';
import ConnectToServer from './components/ConnectToServer.vue'
import { serverConnection, savedIp, savedPassword } from './models/ServerConnection';
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
    position: fixed;
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
    margin: 0;
    padding: 15px;
    overflow-y: auto;
}

@media (max-width: 540px) {
    .main-container {
        position: absolute;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
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

@media (max-width: 540px) {
    h1 {
        font-size: 16px;
    }

    h2 {
        font-size: 12px;
    }
}

/* Toggle styling */
.checkbox-wrapper {
    margin-right: 5px;
}
.checkbox-wrapper input[type="checkbox"] {
    visibility: hidden;
    display: none;
}

.checkbox-wrapper .toggle {
    position: relative;
    display: block;
    width: 40px;
    height: 20px;
    cursor: pointer;
    -webkit-tap-highlight-color: transparent;
    transform: translate3d(0, 0, 0);
}
.checkbox-wrapper .toggle:before {
    content: "";
    position: relative;
    top: 3px;
    left: 3px;
    width: 34px;
    height: 14px;
    display: block;
    background: #9A9999;
    border-radius: 8px;
    transition: background 0.2s ease;
}
.checkbox-wrapper .toggle span {
    position: absolute;
    top: 0;
    left: 0;
    width: 20px;
    height: 20px;
    display: block;
    background: white;
    border-radius: 10px;
    box-shadow: 0 3px 8px rgba(154, 153, 153, 0.5);
    transition: all 0.2s ease;
}
.checkbox-wrapper .toggle span:before {
    content: "";
    position: absolute;
    display: block;
    margin: -18px;
    width: 56px;
    height: 56px;
    background: var(--primary-light-color);
    border-radius: 50%;
    transform: scale(0);
    opacity: 1;
    pointer-events: none;
}

.checkbox-wrapper input:checked + .toggle:before {
    background: var(--primary-dark-color);
}
.checkbox-wrapper input:checked + .toggle span {
    background: var(--primary-color);
    transform: translateX(20px);
    transition: all 0.2s cubic-bezier(0.8, 0.4, 0.3, 1.25), background 0.15s ease;
    box-shadow: 0 3px 8px rgba(220, 46, 46, 0.2);
}
.checkbox-wrapper input:checked + .toggle span:before {
    transform: scale(1);
    opacity: 0;
    transition: all 0.4s ease;
}

button {
    background-color: var(--primary-color);
    color: var(--text-color);
    border: none;
    padding: 10px 20px;
    border-radius: 5px;
    cursor: pointer;
    font-size: 14px;
    transition: background-color 0.3s ease;
}
button:hover {
    background-color: var(--primary-dark-color);
}
button:disabled {
    background-color: var(--disabled-color);
    cursor: not-allowed;
    opacity: 0.6;
}

.spinner {
    display: inline-block;
    width: 16px;
    height: 16px;
    border: 2px solid var(--text-color);
    border-top: 2px solid transparent;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-right: 8px;
    vertical-align: middle;
}

@keyframes spin {
    from {
        transform: rotate(0deg);
    }
    to {
        transform: rotate(360deg);
    }
}

/* Minimalist scrollbars */
::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}

::-webkit-scrollbar-thumb {
    background-color: var(--background-light-color);
    border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
    background-color: var(--background-color);
}

::-webkit-scrollbar-track {
    background-color: var(--background-dark-color);
    border-radius: 4px;
}
</style>