<template>
    <div class="file-explorer">
        <div class="tool-bar">
            <button class="btn" @click="createNewFolder">
                <i class="fa-solid fa-folder-plus"></i>
                <span>Create Folder</span>
            </button>
            <button class="btn">
                <i class="fa-solid fa-file"></i>
                <span>Create File</span>
            </button>
            <button class="btn">
                <i class="fa-solid fa-upload"></i>
                <span>Upload File</span>
            </button>
            <button class="btn" :disabled="!selectedFileName">
                <i class="fa-solid fa-download"></i>
                <span>Download</span>
            </button>
            <button class="btn" :disabled="!selectedFileName" @click="deleteSelection">
                <i class="fa-solid fa-trash"></i>
                <span>Delete</span>
            </button>
            <button class="btn">
                <i class="fa-solid fa-rotate"></i>
                <span>Refresh</span>
            </button>
        </div>
        <div class="file-icons">
            <div 
                v-for="file in files" 
                :key="file.name" 
                class="file-item"
                :class="{ selected: file.name === selectedFileName }"
                @click="selectFile(file.name)"
            >
                <i :class="file.icon"></i>
                <span>{{ file.name }}</span>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const files = ref([
    { name: 'Document.txt', icon: 'fa-solid fa-file-alt' },
    { name: 'Image.png', icon: 'fa-solid fa-file-image' },
    { name: 'Video.mp4', icon: 'fa-solid fa-file-video' },
    { name: 'Audio.mp3', icon: 'fa-solid fa-file-audio' },
    { name: 'Archive.zip', icon: 'fa-solid fa-file-archive' },
]);

const selectedFileName = ref('');
const selectFile = (fileName: string) => {
    selectedFileName.value = fileName;
};

function deleteSelection() {
    if (selectedFileName.value) {
        files.value = files.value.filter(file => file.name !== selectedFileName.value);
        selectedFileName.value = '';
    }
}

function createNewFolder() {
    files.value.push({ name: 'New Folder', icon: 'fa-solid fa-folder' });
}

</script>

<style scoped>
.tool-bar {
    display: flex;
    justify-content: space-between;
}

.file-icons {
    display: flex;
    flex-wrap: wrap;
    padding: 10px;
}
.file-item {
    display: flex;
    align-items: center;
    flex-direction: column;
    margin: 10px;
    padding: 20px;
    border-radius: 5px;
    cursor: pointer;
}
.file-item.selected {
    background-color: var(--primary-color);
    color: white;
}
.file-item i {
    margin-bottom: 20px;
    font-size: 48px;
    color: var(--text-color);
}
.file-item span {
    font-size: 14px;
    color: var(--text-color);
    user-select: none;
}

</style>