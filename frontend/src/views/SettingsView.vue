<template>
    <h2 id="cpu-alerts">CPU Temperature Alerts & Actions</h2>
    <form>
        <!-- Warning Notifications-->
        <div class="form-group">
            <div class="checkbox-wrapper">
                <input v-model="warningEnabled" type="checkbox" id="checkbox" />
                <label for="checkbox" class="toggle"><span></span></label>
            </div>
            <label>Enable Warning Notification</label>
            <i class="fa-solid fa-circle-info">
                <p class="tooltip">Receive warning notifications when your server reaches or excedes a specified temperature via the Alertzy app.</p>
            </i>
        </div>
        <div class="sub-group" :class="{'disabled': !warningEnabled}">
            <label for="warningTemp">Set Warning Temperature (°C)</label><br/>
            <input v-model="warningTemp" type="number" min="40" max="150">
        </div>

        <!-- Automatic Shutdown -->
        <div class="form-group">
            <div class="checkbox-wrapper">
                <input v-model="shutdownEnabled" type="checkbox" id="shutdownCheckbox" />
                <label for="shutdownCheckbox" class="toggle"><span></span></label>
            </div>
            <label>Enable Automatic Shutdown</label>
            <i class="fa-solid fa-circle-info">
                <p class="tooltip">Automatically shutdown the server and receieve a notification via the Alertzy app when your server reaches or excedes a specified temperature.</p>
            </i>
        </div>
        <div class="sub-group" :class="{'disabled': !shutdownEnabled}">
            <label for="shutdownTemp">Set Shutdown Temperature (°C)</label><br />
            <input v-model="shutdownTemp" type="number" min="40" max="150">
        </div>
        <button @click.prevent="updateTempConfig" :class="{'loading': isSaving}">
            <span v-if="isSaving" class="spinner"></span>
            <span v-else>Save Settings</span>
        </button>
    </form>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { serverConnection } from '../models/ServerConnection';

const warningEnabled = ref(false);
const warningTemp = ref(70);
const shutdownEnabled = ref(false);
const shutdownTemp = ref(80);
const isSaving = ref(false);

async function updateTempConfig() {
    isSaving.value = true;
    console.log("Saving settings...");
    const connection = serverConnection.value;
    if (connection) { await connection.setTempConfig(warningTemp.value, shutdownTemp.value, warningEnabled.value, shutdownEnabled.value); }
    console.log("Settings saved.");
    isSaving.value = false;
}

const tempConfig = computed(() => {
    const connection = serverConnection.value;
    if (connection) {
        return {
            warningTemp: connection.getTempConfig("warning_temp"),
            shutdownTemp: connection.getTempConfig("shutdown_temp"),
            warningsEnabled: connection.getTempConfig("warnings_enabled"),
            shutdownsEnabled: connection.getTempConfig("shutdown_enabled")
        };
    }
    return null;
});

watch(tempConfig, (newConfig) => {
    if (newConfig) {
        warningTemp.value = newConfig.warningTemp;
        shutdownTemp.value = newConfig.shutdownTemp;
        warningEnabled.value = newConfig.warningsEnabled;
        shutdownEnabled.value = newConfig.shutdownsEnabled;
    }
}, { immediate: true });
</script>

<style scoped>
h2 {
    margin-bottom: 10px;
}
.form-group {
    display: flex;
    align-items: center;
    width: 100%;
    margin-bottom: 5px;
    padding-bottom: 10px;
    border-bottom: 1px solid var(--border-color);
}
.form-group i {
    margin-left: 10px;
    color: var(--text-color);
    cursor: pointer;
    text-align: center;
    display: flex;
    align-items: center;
    justify-content: center;
}

.form-group i:hover .tooltip {
    opacity: 1;
}

.tooltip {
    opacity: 0;
    position: absolute;
    background-color: var(--primary-color);
    color: var(--tooltip-text-color);
    padding: 5px;
    border-radius: 5px;
    font-size: 12px;
    z-index: 1000;
    width: 200px;
    transform: translateY(80%);
    transition: opacity 0.3s ease;
    pointer-events: none;
    line-height: 1.2;
}

.sub-group {
    margin-left: 20px;
    margin-bottom: 20px;
    display: flex;
    flex-direction: row;
    align-items: center;
}
label {
    color: var(--text-color);
    font-size: 14px;
    margin-right: 15px;
}

input[type="checkbox"] {
    margin-right: 10px;
    cursor: pointer;
}

input[type="number"] {
    width: 60px;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    padding: 5px;
    font-size: 14px;
}

div.disabled {
    opacity: 0.3;
    pointer-events: none;
}

</style>