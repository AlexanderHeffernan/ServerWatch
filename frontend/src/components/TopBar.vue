<template>
    <div class="top-bar">
        <i class="mobile-icon fa-solid fa-bars" @click="$emit('mobile-sidebar-toggle')"></i>
        <div class="server-name">
            <h2 @click="toggleServerDropdown">Alex's Raspberry Pi</h2>
            <i @click="toggleServerDropdown" class="fa-solid fa-chevron-down" :class="{ 'rotate': isServerDropdownVisible }"></i>
            <div class="server-dropdown" :class="{ 'show': isServerDropdownVisible }">
                <p>Server Options</p>
                <a>Disconnect</a>
                <a>Add Server</a>
                <hr />
                <p>Your Servers</p>
                <a>Alex's Raspberry Pi</a>
            </div>
        </div>
        <div class="quick-actions">
            <i @click="handleRefresh" class="fa-solid fa-arrows-rotate" :class="{ 'refreshing': isRefreshing }" id="refresh-icon"></i>
            <div class="dropdown">
                <i class="fa-solid fa-bell" id="notification-icon"></i>
                <div class="dropdown-menu">
                    <p v-if="!notificationsManager?.notifications">No new notifications</p>
                    <div v-else>
                        <p>Notifications</p>
                        <div
                            v-for="notification in notificationsManager?.notifications" 
                            :key="notification.id" 
                            :class="{ [notification.type]: true, 'open': openNotificationId === notification.id }"
                            @click="openNotification(notification.id)"
                            class="notification"
                        >
                            <div class="notification-header">
                                <i v-if="notification.type === 'error'" class="fa-solid fa-triangle-exclamation"></i>
                                <span>{{ notification.title }}</span>
                            </div>
                            <span class="description">{{ notification.message }}</span>
                        </div>
                    </div>
                </div>
            </div>
            <div class="dropdown">
                <i class="fa-solid fa-power-off" id="power-icon" style="color: var(--accent-light-color)"></i>
                <div class="dropdown-menu">
                    <a @click="handleShutdown">Shutdown</a>
                    <a @click="handleReboot">Reboot</a>
                </div>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import { serverConnection } from '../models/ServerConnection';
import { notificationsManager } from '../models/NotificationsManager';

const isServerDropdownVisible = ref(false);
const isRefreshing = ref(false);

const openNotificationId = ref("");

function openNotification (notificationId: string) {
    if (openNotificationId.value === notificationId) {
        openNotificationId.value = "";
    } else {
        openNotificationId.value = notificationId;
    }
}

function toggleServerDropdown() {
    isServerDropdownVisible.value = !isServerDropdownVisible.value;
}

async function handleRefresh() {
    isRefreshing.value = true;
    await serverConnection.value?.refresh();
    isRefreshing.value = false;
}

function handleShutdown() {
    serverConnection.value?.shutdown();
}

function handleReboot() {
    serverConnection.value?.reboot();
}

</script>
<style scoped>
.top-bar {
    width: 100%;
    height: 65px;
    border-bottom: 1px solid var(--border-color);
    padding: 0 15px 0 15px;
    display: flex;
    align-items: center;
    background-color: var(--background-color);
}

.mobile-icon {
    display: none;
}
@media (max-width: 540px) {
    .mobile-icon {
        display: block;
        color: #FFFFFF;
        font-size: 20px;
        cursor: pointer;
        margin-right: 10px;
    }

    .server-name {
        margin-right: 0 !important;
        max-width: 40%;
    }

    .quick-actions i {
        padding: 0 10px !important;
    }
}

.server-name {
    display: flex;
    align-items: center;
    gap: 5px;
    flex-grow: 1;
    margin-right: 35px;
    position: relative;
    height: 100%;
}

.server-name h2 {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    cursor: pointer;
}

.server-name i {
    color: #FFFFFF;
    font-size: 16px;
    font-weight: 900;
    cursor: pointer;
    transition: transform 0.2s ease;
}

.server-name i.rotate {
    transform: rotate(180deg);
}

.server-dropdown {
    max-height: 0;
    overflow: hidden;
    position: absolute;
    top: calc(100% + 1px);
    left: -16px;
    background-color: var(--background-dark-color);
    color: var(--text-color);
    border-radius: 0 0 13px 13px;
    white-space: nowrap;
    transition: max-height 0.2s ease, padding 0.2s ease;
    border: 0;
    width: 200px;
}

.server-dropdown.show {
    max-height: 200px;
    padding: 10px 0;
    border-style: solid;
    border-color: var(--border-color);
    border-width: 0 1px 1px 1px;
}

.server-dropdown hr {
    border: none;
    border-top: 1px solid var(--border-color);
    margin: 10px 0;
}

.server-dropdown p {
    margin: 0;
    padding: 5px 10px;
}

.server-dropdown a {
    display: block;
    width: 100%;
    padding: 10px;
    text-align: left;
    background: none;
    border: none;
    cursor: pointer;
    position: relative;
    transition: color 0.3s ease;
}

.server-dropdown a:hover {
    color: var(--primary-color);
}

.quick-actions {
    margin-left: auto;
    display: flex;
    align-items: center;
    height: 100%;
}

.quick-actions i {
    color: #FFFFFF;
    font-size: 16px;
    font-weight: 900;
    cursor: pointer;
    padding: 0 15px;
}

.dropdown {
    position: relative;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
}

.dropdown-menu {
    max-height: 0;
    overflow: hidden;
    position: absolute;
    top: calc(100% + 1px);
    right: -15px;
    background-color: var(--background-dark-color);
    color: var(--text-color);
    border-radius: 0 0 13px 13px;
    white-space: nowrap;
    transition: max-height 0.2s ease, padding 0.2s ease;
    border: 0;
    overflow-y: auto;
}

.dropdown:hover .dropdown-menu {
    max-height: 200px;
    padding: 10px 0;
    border-style: solid;
    border-color: var(--border-color);
    border-width: 0 1px 1px 1px;
}

.dropdown:hover i#power-icon {
    text-shadow: 0 0 15px var(--accent-light-color), /* Inner glow */
                 0 0 20px var(--accent-light-color), /* Outer glow */
                 0 0 25px var(--accent-light-color); /* Stronger outer glow */
    transition: text-shadow 0.3s ease;
}

.dropdown:hover i#notification-icon {
    animation: rotate-shake 0.5s ease;
}

i#refresh-icon {
    transition: opacity 0.3s ease, transform 0.5s ease;
    opacity: 1;
}

i#refresh-icon:hover:not(.reloading) {
    animation: rotate-spin 0.5s ease;
}

i#refresh-icon.refreshing {
    animation: rotate-spin 0.5s linear infinite !important;
    opacity: 0.5;
    pointer-events: none;
}

i#refresh-icon:not(.refreshing) {
    transform: rotate(0deg); /* Ensures it smoothly transitions back to its normal rotation */
}

.dropdown-menu a, .dropdown-menu a:visited {
    display: block;
    margin: 0;
    padding: 10px 20px;
    cursor: pointer;
    position: relative;
    transition: color 0.3s ease;
    text-decoration: none;;
    color: var(--text-color)
}

.dropdown-menu a::after, .server-dropdown a::after {
    content: ''; /* Creates the underline */
    position: absolute;
    bottom: 0; /* Position the underline at the bottom of the link */
    left: 0;
    width: 0; /* Start with no underline */
    height: 2px; /* Thickness of the underline */
    background-color: var(--primary-color); /* Color of the underline */
    transition: width 0.3s ease; /* Smooth transition for the underline */
}

.dropdown-menu a:hover {
    color: var(--primary-color);
    /* color: #FFFFFF; */
}

.dropdown-menu a:hover::after, .server-dropdown a:hover::after {
    width: 100%; /* Expand the underline to full width */
}

@keyframes rotate-shake {
    0% { transform: rotate(0); }
    25% { transform: rotate(-10deg); }
    50% { transform: rotate(10deg); }
    75% { transform: rotate(-10deg); }
    100% { transform: rotate(0); }
}

@keyframes rotate-spin {
    0% { transform: rotate(0); }
    100% { transform: rotate(360deg); }
}

.dropdown-menu p {
    margin: 0;
    padding: 10px 20px;
}

.notification {
    padding: 10px 20px;
    cursor: pointer;
    transition: background-color 0.3s ease;
    margin: 10px 20px;
    border-radius: 12px;
    width: 250px;
}

.notification.error {
    background-color: var(--primary-color);
    color: var(--text-color);
}

.notification.error:hover {
    background-color: var(--primary-dark-color);
}

.notification .notification-header {
    display: flex;
    align-items: center;
    gap: 10px;
}

.notification .notification-header i {
    margin: 0;
    padding: 0;
}

.notification .notification-header span {
    margin: 0;
    padding: 0;
    font-size: 14px;
    white-space: normal;
    word-wrap: break-word;
    line-height: 1.2;
}

.notification .description {
    font-size: 12px;
    color: var(--text-color);
    max-height: 0;
    overflow: hidden;
    transition: max-height 0.2s ease, margin 0.2s ease;
    display: block;
    margin-top: 0px;
    white-space: normal;
    word-wrap: break-word;
    line-height: 1.2;
}

.notification.open .description {
    max-height: 50px;
    margin-top: 5px;
}
</style>