<template>
    <div class="reconnect-dialog-overlay">
        <div class="reconnect-dialog">
            <h2>Rebooting Server</h2>
            <p>Please wait while we try to reconnect you with your server.</p>
            <i class="fa-solid fa-spinner fa-spin"></i>
            <p>This may take a moment.</p>
            <button class="cancel-button" @click="onCancel">Manually Reconnect</button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from 'vue';
import { savedIp, savedPassword, savedDemoMode, initiateServerConnection, initiateDemoConnection } from '../models/ServerConnection';

const reconnectInterval = ref<number | null>(null);

async function attemptReconnect() {
    if (savedIp.value && savedPassword.value && !savedDemoMode.value) {
        try {
            const controller = new AbortController();
            const timeoutId = setTimeout(() => controller.abort(), 5000); // Prevents hanging

            const response = await fetch(`https://${savedIp.value}/test-connection?password=${savedPassword.value}`, {
                method: 'GET',
                signal: controller.signal,
                headers: {
                    "ngrok-skip-browser-warning": "true" // Bypass ngrok’s free tier warning
                }
            });

            clearTimeout(timeoutId);

            if (!response.ok) {
                throw new Error('Failed to connect to the server.');
            }

            let serverName = 'Unknown Server';
            try {
                const data = await response.json();
                serverName = data.server_name || serverName;
            } catch {
                console.log('Failed to parse server name from response.');
            }
            initiateServerConnection(savedIp.value, savedPassword.value, serverName);
        } catch (error) { console.log('Reconnect attempt failed:', error); }
    } else if (savedDemoMode.value) {
        // wait 2.5 seconds
        setTimeout(() => { initiateDemoConnection(); }, 2500);
    }
}

function onCancel() {
    if (reconnectInterval.value) {
        clearInterval(reconnectInterval.value);
    }
    savedIp.value = '';
    savedPassword.value = '';
    savedDemoMode.value = false;
}

onMounted(() => {
    if (savedIp.value && savedPassword.value) {
        reconnectInterval.value = setInterval(attemptReconnect, 5000);
    }
});

onBeforeUnmount(() => {
    if (reconnectInterval.value) {
        clearInterval(reconnectInterval.value);
    }
})

</script>
<style scoped>
.reconnect-dialog-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
    backdrop-filter: blur(5px);
}

.reconnect-dialog {
    background-color: var(--background-color);
    padding: 20px;
    border-radius: 8px;
    text-align: center;
    box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.3);
    border: 1px solid var(--border-color);
}

.reconnect-dialog h2 {
    color: var(--primary-color);
    margin-bottom: 20px;
}

.reconnect-dialog p {
    color: var(--text-color);
    margin-bottom: 10px;
}

.reconnect-dialog i {
    font-size: 24px;
    color: var(--primary-color);
    margin-bottom: 10px;
}

.reconnect-dialog .cancel-button {
    background-color: var(--background-lighter-color);
    color: var(--text-color);
    border: none;
    padding: 10px 20px;
    border-radius: 4px;
    cursor: pointer;
    margin-top: 10px;
}

.reconnect-dialog .cancel-button:hover {
    background-color: var(--background-light-color);
}
</style>