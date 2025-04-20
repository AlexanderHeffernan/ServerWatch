<template>
    <div :class="['side-nav', { minimized, 'mobile-sidebar-minimized': mobileSidebarMinimized }]">
        <div class="header-container">
            <img src="../assets/icon.png" alt="Logo" @click="toggleSidebar" />
            <h1 :class="{ hidden: minimized}">ServerWatch</h1>
            <i 
                class="fa-solid fa-chevron-left" 
                :class="{ hidden: minimized, 'mobile': !mobileSidebarMinimized}"
                @click="toggleSidebar">
            </i>
        </div>
        <div class="nav-links">
            <router-link to="/" class="nav-link" active-class="active">
                <i class="fa-solid fa-gauge"></i>
                <span :class="{ hidden: minimized }">Dashboard</span>
            </router-link>
            <router-link to="/system" class="nav-link disabled" active-class="active">
                <i class="fa-solid fa-server"></i>
                <span :class="{ hidden: minimized }">System</span>
            </router-link>
            <router-link to="/processes" class="nav-link disabled" active-class="active">
                <i class="fa-solid fa-gears"></i>
                <span :class="{ hidden: minimized }">Processes</span>
            </router-link>
            <router-link to="/network" class="nav-link disabled" active-class="active">
                <i class="fa-solid fa-network-wired"></i>
                <span :class="{ hidden: minimized }">Network</span>
            </router-link>
            <router-link to="/storage" class="nav-link disabled" active-class="active">
                <i class="fa-solid fa-hard-drive"></i>
                <span :class="{ hidden: minimized }">Storage</span>
            </router-link>
            <router-link to="/services" class="nav-link disabled" active-class="active">
                <i class="fa-solid fa-wrench"></i>
                <span :class="{ hidden: minimized }">Services</span>
            </router-link>
            <router-link to="/logs" class="nav-link disabled" active-class="active">
                <i class="fa-regular fa-file-lines"></i>
                <span :class="{ hidden: minimized }">Logs</span>
            </router-link>
            <router-link to="/settings" class="nav-link" active-class="active">
                <i class="fa-solid fa-gear"></i>
                <span :class="{ hidden: minimized }">Settings</span>
            </router-link>
        </div>
    </div>
</template>
<script setup lang="ts">
import { ref, defineEmits, inject } from 'vue';

const minimized = ref(false);
const mobileSidebarMinimized = inject('mobileSidebarMinimized');
const emit = defineEmits(['sidebar-toggled']);

function toggleSidebar() {
    if (window.innerWidth > 480) {
        minimized.value = !minimized.value;   
    }
    emit('sidebar-toggled');
}

window.addEventListener('resize', () => {
    if (window.innerWidth <= 480) {
        minimized.value = false;
    }
});
</script>
<style scoped>
.side-nav {
    background-color: #181E20;
    height: 100%;
    width: 250px;
    padding: 15px;
    transition: width 0.4s ease, padding 0.4s ease;
    border-right: 1px solid var(--border-color);
}

.side-nav.minimized {
    width: 60px;
    padding: 15px 10px 15px 10px;
}

.header-container {
    display: flex;
    align-items: center;
    gap: 10px;
}

.header-container img {
    width: 35px;
    height: 35px;
}

.header-container h1, .header-container i {
    transition: opacity 0.3s ease, visibility 0.3s ease;
}

.header-container h1.hidden, .header-container i.hidden {
    opacity: 0;
    visibility: hidden;
}

.header-container i {
    color: #FFFFFF;
    font-size: 16px;
    font-weight: 900;
    cursor: pointer;
}

.header-container i.mobile {
    margin-left: auto;
}

.nav-links {
    margin-top: 20px;
    display: flex;
    flex-direction: column;
}

.nav-links .nav-link {
    display: flex;
    align-items: center;
    gap: 10px;
    text-decoration: none;
    color: #FFFFFF;
    font-size: 14px;
    font-weight: 600;
    padding: 10px;
    border-radius: 18px;
}

.nav-links .nav-link.active {
    background-color: var(--primary-color);
}

.nav-link.disabled {
    pointer-events: none;
    opacity: 0.3;
}

.nav-link i {
    width: 20px;
    text-align: center;
    flex-shrink: 0;
    flex-grow: 0;
}

.nav-link span {
    white-space: nowrap;
    transition: opacity 0.3s ease, visibility 0.3s ease;
}

.nav-link span.hidden {
    opacity: 0;
    visibility: hidden;
}

@media (max-width: 540px) {
    .side-nav {
        position: absolute;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        padding: 15px;
        z-index: 9999;
        transition: left 0.4s ease, width 0.4s ease, padding 0.4s ease;
    }
    
    .side-nav.mobile-sidebar-minimized {
        position: relative;
        left: -100% !important;
        width: 0 !important;
    }
}
</style>