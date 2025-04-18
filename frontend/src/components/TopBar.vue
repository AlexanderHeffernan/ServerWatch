<template>
    <div class="top-bar">
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

.server-name {
    display: flex;
    align-items: center;
    gap: 10px;
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