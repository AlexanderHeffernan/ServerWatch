<template>
    <SideNav />
    <div class="main-container">
        <TopBar />
        <div class="content">
            <router-view />
        </div>
    </div>
</template>

<script lang="ts" setup>
import SideNav from './components/SideNav.vue';
import TopBar from './components/TopBar.vue';
</script>

<style>
:root {
    --primary-color: #C51A4A;
    --primary-dark-color: #5D1A2A;
    --primary-light-color: #E0557A;
    --background-color: #263238;
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

h1 {
    color: #FFFFFF;
    font-size: 18px;
}
</style>