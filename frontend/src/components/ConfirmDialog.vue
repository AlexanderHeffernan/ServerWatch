<template>
    <div v-if="visible" class="confirm-dialog-overlay">
        <div class="confirm-dialog">
            <h2>{{ message }}</h2>
            <div class="actions">
                <button @click="onConfirm" class="confirm-button">Yes</button>
                <button @click="onCancel" class="cancel-button">No</button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';

defineProps({
    visible: Boolean,
    message: String,
});

const emit = defineEmits(['confirm', 'cancel']);
const onConfirm = () => { emit('confirm'); };
const onCancel = () => { emit('cancel'); };
</script>

<style scoped>
.confirm-dialog-overlay {
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

.confirm-dialog {
    background-color: var(--background-color);
    padding: 20px;
    border-radius: 8px;
    text-align: center;
    box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.3);
    border: 1px solid var(--border-color);
}

.confirm-dialog h2 {
    color: var(--text-color);
    margin-bottom: 20px;
}

.confirm-dialog .actions button {
    margin: 0 10px;
    padding: 10px 20px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.confirm-dialog .actions button.confirm-button {
  background-color: var(--primary-color);
  color: white;
}

.confirm-dialog .actions button.confirm-button:hover {
  background-color: var(--primary-dark-color);
}

.confirm-dialog .actions button.cancel-button {
  background-color: var(--background-lighter-color);
  color: var(--text-color);
}

.confirm-dialog .actions button.cancel-button:hover {
  background-color: var(--background-light-color);
}
</style>