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
                <i class="fa-solid fa-bell" id="notification-icon" :class="{ 'pulse-error': pulseNotification}">
                    <span v-if="notificationsManager?.notifications?.length" class="notification-badge">{{ notificationsManager?.notifications?.length }}</span>
                </i>
                <div class="dropdown-menu notification-dropdown">
                    <p v-if="!notificationsManager?.notifications">No new notifications</p>
                    <div v-else>
                        <p style="font-weight: bold; padding-bottom: 0px;">Notifications</p>
                        <p v-if="notificationsManager?.notifications?.length === 0">No new notifications</p>
                        <div
                            
                            v-for="notification in notificationsManager?.notifications" 
                            :key="notification.id" 
                            :class="{ [notification.type]: true, 'open': openNotificationId === notification.id }"
                            @click="openNotification(notification.id)"
                            class="notification"
                            :data-id="notification.id"
                        >
                            <div class="notification-header">
                                <i v-if="notification.type === 'error'" class="fa-solid fa-triangle-exclamation type-icon"></i>
                                <div class="main-container">
                                    <span class="title">{{ notification.title }}</span>
                                    <span class="timestamp">
                                        {{ new Date(notification.timestamp).toDateString() === new Date().toDateString() 
                                            ? notification.formattedTimestamp 
                                            : notification.formattedDate }}
                                    </span>
                                </div>
                                <div 
                                    class="delete-icon-container"
                                    @click="deleteNotification(notification.id)"
                                    @click.stop
                                >
                                    <i class="fa-solid fa-trash-can delete-icon"></i>
                                </div>
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
import { ref, watch } from 'vue';
import { serverConnection } from '../models/ServerConnection';
import { notificationsManager } from '../models/NotificationsManager';

const isServerDropdownVisible = ref(false);
const isRefreshing = ref(false);

const openNotificationId = ref("");
const pulseNotification = ref(false);

function openNotification (notificationId: string) {
    if (openNotificationId.value === notificationId) {
        openNotificationId.value = "";
    } else {
        openNotificationId.value = notificationId;
    }
}

function deleteNotification(notificationId: string) {
    const notificationElement = document.querySelector(`.notification[data-id="${notificationId}"]`);
    console.log("Hello", notificationElement);
    if (notificationElement) {
        notificationElement.classList.remove('open');
        notificationElement.classList.add('fade-out');
        setTimeout(() => {
            notificationsManager.value?.removeNotification(notificationId);
        }, 300); // Match this with the CSS transition duration
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

let oldNotifications = [];

watch(
    () => notificationsManager.value?.notifications,
    (newNotifications) => {
        console.log('Watcher triggered');
        console.log('New notifications:', newNotifications?.length);
        console.log('Old notifications:', oldNotifications.length);

        if ((newNotifications?.length ?? 0) > oldNotifications?.length) {
            const latestNotification = newNotifications?.[newNotifications.length - 1];
            if (latestNotification?.type === 'error') {
                pulseNotification.value = true;
                console.log("Pulse notification triggered");
                setTimeout(() => (pulseNotification.value = false), 1000);
            }
        }

        oldNotifications = [...(newNotifications || [])];
    },
    { deep: true }
);

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

.dropdown-menu.notification-dropdown {
    width: 291px;
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

.dropdown:hover i#notification-icon:not(.pulse-error) {
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
    cursor: pointer;
    transition: background-color 0.3s ease;
    margin: 10px 20px;
    border-radius: 12px;
    width: 250px;
    background-color: var(--background-color);
    color: var(--text-color);
    border: 1px solid var(--border-color);
    transition: border 0.3s ease;
}

.notification:hover {
    border: 1px solid var(--primary-color);
}

.notification .notification-header {
    display: flex;
    align-items: center;
    gap: 10px;
    height: 58px;
}

.notification .notification-header .main-container {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    width: 100%;
    position: relative;
}

.notification .notification-header .top-section {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-grow: 1;
}

.notification .notification-header i {
    margin: 0;
    padding: 0;
}

.notification.error .notification-header .type-icon {
    color: var(--primary-color);
    font-size: 16px;
    margin-left: 10px;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
}

.notification .notification-header .title {
    margin: 0;
    padding: 0;
    font-size: 12px;
    white-space: normal;
    word-wrap: break-word;
    line-height: 1.2;
    display: flex;
    align-items: center;
    flex-grow: 1;
}

.notification .notification-header .timestamp {
    font-size: 10px;
    color: var(--background-lighter-color);
    font-weight: 400;
    margin-left: auto;
    position: absolute;
    right: 0;
    bottom: 5px;
}

.notification .notification-header .delete-icon-container {
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--primary-color);
    /* border-left: 1px solid var(--border-color); */
    height: 100%;
    padding: 0 10px;
    border-radius: 0 11px 11px 0;
    transition: color 0.3s ease, background-color 0.3s ease, border-radius 0.3s ease;
}

.notification.open .notification-header .delete-icon-container {
    border-radius: 0 11px 0 0;
}

.notification .notification-header .delete-icon-container .delete-icon {
    color: var(--primary-dark-color);
    font-size: 12px;
    cursor: pointer;
    transition: color 0.3s ease;
}

.notification .notification-header .delete-icon-container:hover {
    background-color: var(--primary-dark-color);
}

.notification .notification-header .delete-icon-container:hover .delete-icon {
    color: var(--primary-color);
}

.notification:has(.notification-header .delete-icon-container:hover) {
    border: 1px solid var(--primary-dark-color);
}

.notification:has(.notification-header .delete-icon-container:hover) .title,
.notification:has(.notification-header .delete-icon-container:hover) .type-icon,
.notification:has(.notification-header .delete-icon-container:hover) .timestamp,
.notification:has(.notification-header .delete-icon-container:hover) .description {
    opacity: 0.2;
    transition: opacity 0.3s ease;
}



.notification .description {
    font-size: 12px;
    color: var(--text-color);
    max-height: 0;
    overflow: hidden;
    transition: max-height 0.2s ease, margin 0.2s ease, border 0.3s ease, padding 0.2s ease;
    display: block;
    margin-top: 0px;
    white-space: normal;
    word-wrap: break-word;
    line-height: 1.2;
    border-top: solid 0px var(--background-color);
    padding: 0px 5px 0px 10px;
}

.notification.open .description {
    max-height: 100px;
    padding: 10px 5px 10px 10px;
    border-top: solid 1px var(--border-color);
}

.notification.fade-out {
    transform: scaleX(0);
    transform-origin: 0;
    transition: transform 0.2s ease;
}

#notification-icon {
    display: inline-block;
    transition: text-shadow 0.3s ease, transform 0.5s ease, color 0.3s ease;
    position: relative;
}

.pulse-error {
    /* animation: pulse-grow 1s ease-in-out; */
    text-shadow: 0 0 15px var(--primary-color), /* Inner glow */
                 0 0 20px var(--primary-color), /* Outer glow */
                 0 0 25px var(--primary-color);
    animation: rotate-shake-scale 0.5s ease;
    animation-iteration-count: infinite;
    color: var(--primary-color) !important;
}

@keyframes rotate-shake-scale {
    0% { transform: rotate(0) scale(1.4); }
    25% { transform: rotate(-10deg) scale(1.4); }
    50% { transform: rotate(10deg) scale(1.4); }
    75% { transform: rotate(-10deg) scale(1.4); }
    100% { transform: rotate(0) scale(1.4); }
}

#notification-icon .notification-badge {
    position: absolute;
    top: 10px;
    right: -4px;
    background-color: var(--primary-color);
    color: var(--text-color);
    border-radius: 50%;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    font-weight: bold;
    line-height: 1;
    text-align: center;
    box-sizing: border-box;
}
</style>