<template>
    <div class="top-bar">
        <i class="mobile-icon fa-solid fa-bars" @click="$emit('mobile-sidebar-toggle')"></i>
        <div class="server-name">
            <h2>Alex's Raspberry Pi</h2>
            <i class="fa-solid fa-chevron-down"></i>
        </div>
        <div class="quick-actions">
            <i class="fa-solid fa-bell"></i>
            <i class="fa-solid fa-power-off" style="color: var(--accent-color)"></i>
        </div>
    </div>
</template>
<script setup lang="ts">

</script>
<style scoped>
.top-bar {
    width: 100%;
    height: 65px;
    border-bottom: 1px solid #4C5159;
    padding: 15px;
    display: flex;
    align-items: center;
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
    display: flex;
    align-items: center;
    gap: 20px;
}

.quick-actions i {
    color: #FFFFFF;
    font-size: 16px;
    font-weight: 900;
    cursor: pointer;
}
</style>