<template>
    <div class="top-bar">
        <i class="mobile-icon fa-solid fa-bars" @click="$emit('mobile-sidebar-toggle')"></i>
        <div class="server-name">
            <h2>Alex's Raspberry Pi</h2>
            <i class="fa-solid fa-chevron-down"></i>
        </div>
        <div class="quick-actions">
            <div class="dropdown">
                <i class="fa-solid fa-bell" id="notification-icon"></i>
                <div class="dropdown-menu">
                    <p>No new notifications</p>
                    <a>Notification Settings</a>
                </div>
            </div>
            <div class="dropdown">
                <i class="fa-solid fa-power-off" id="power-icon" style="color: var(--accent-light-color)"></i>
                <div class="dropdown-menu">
                    <a>Shutdown</a>
                    <a>Reboot</a>
                </div>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">

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
@media (max-width: 480px) {
    .mobile-icon {
        display: block;
        color: #FFFFFF;
        font-size: 20px;
        cursor: pointer;
        margin-right: 10px;
    }
}

.server-name {
    display: flex;
    align-items: center;
    gap: 5px;
    flex-grow: 1;
    overflow: hidden;
    margin-right: 35px;
}

.server-name h2 {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.server-name i {
    color: #FFFFFF;
    font-size: 16px;
    font-weight: 900;
    cursor: pointer;
}

.quick-actions {
    margin-left: auto;
    margin-right: 10px;
    display: flex;
    align-items: center;
    gap: 30px;
    height: 100%;
}

.quick-actions i {
    color: #FFFFFF;
    font-size: 16px;
    font-weight: 900;
    cursor: pointer;
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
    right: -25px;
    background-color: var(--background-dark-color);
    color: var(--text-color);
    border-radius: 0 0 13px 13px;
    padding: 0px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    z-index: 10;
    white-space: nowrap;
    transition: max-height 0.2s ease, padding 0.2s ease;
    border: 0;
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

.dropdown-menu a {
    display: block;
    margin: 0;
    padding: 10px 20px;
    cursor: pointer;
    position: relative;
    transition: background-color 0.3s ease, color 0.3s ease;
}

.dropdown-menu a::after {
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

.dropdown-menu a:hover::after {
    width: 100%; /* Expand the underline to full width */
}

@keyframes rotate-shake {
    0% { transform: rotate(0); }
    25% { transform: rotate(-10deg); }
    50% { transform: rotate(10deg); }
    75% { transform: rotate(-10deg); }
    100% { transform: rotate(0); }
}

.dropdown-menu p {
    margin: 0;
    padding: 10px 20px;
}
</style>