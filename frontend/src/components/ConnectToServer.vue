<template>
    <div class="connect-container">
        <div class="connect-content">
            <img src="../assets/icon.png" alt="Logo" class="logo" />
            <h1>ServerWatch</h1>
            <label for="server-ip">IP Address/Domain</label><br />
            <div class="input-container">
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M352 256c0 22.2-1.2 43.6-3.3 64l-185.3 0c-2.2-20.4-3.3-41.8-3.3-64s1.2-43.6 3.3-64l185.3 0c2.2 20.4 3.3 41.8 3.3 64zm28.8-64l123.1 0c5.3 20.5 8.1 41.9 8.1 64s-2.8 43.5-8.1 64l-123.1 0c2.1-20.6 3.2-42 3.2-64s-1.1-43.4-3.2-64zm112.6-32l-116.7 0c-10-63.9-29.8-117.4-55.3-151.6c78.3 20.7 142 77.5 171.9 151.6zm-149.1 0l-176.6 0c6.1-36.4 15.5-68.6 27-94.7c10.5-23.6 22.2-40.7 33.5-51.5C239.4 3.2 248.7 0 256 0s16.6 3.2 27.8 13.8c11.3 10.8 23 27.9 33.5 51.5c11.6 26 20.9 58.2 27 94.7zm-209 0L18.6 160C48.6 85.9 112.2 29.1 190.6 8.4C165.1 42.6 145.3 96.1 135.3 160zM8.1 192l123.1 0c-2.1 20.6-3.2 42-3.2 64s1.1 43.4 3.2 64L8.1 320C2.8 299.5 0 278.1 0 256s2.8-43.5 8.1-64zM194.7 446.6c-11.6-26-20.9-58.2-27-94.6l176.6 0c-6.1 36.4-15.5 68.6-27 94.6c-10.5 23.6-22.2 40.7-33.5 51.5C272.6 508.8 263.3 512 256 512s-16.6-3.2-27.8-13.8c-11.3-10.8-23-27.9-33.5-51.5zM135.3 352c10 63.9 29.8 117.4 55.3 151.6C112.2 482.9 48.6 426.1 18.6 352l116.7 0zm358.1 0c-30 74.1-93.6 130.9-171.9 151.6c25.5-34.2 45.2-87.7 55.3-151.6l116.7 0z"/></svg>
				<input type="text" v-model="serverIp" placeholder="Type your IP Address">
			</div><br />

            <label for="server-password">Password</label><br />
            <div class="input-container">
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M144 144l0 48 160 0 0-48c0-44.2-35.8-80-80-80s-80 35.8-80 80zM80 192l0-48C80 64.5 144.5 0 224 0s144 64.5 144 144l0 48 16 0c35.3 0 64 28.7 64 64l0 192c0 35.3-28.7 64-64 64L64 512c-35.3 0-64-28.7-64-64L0 256c0-35.3 28.7-64 64-64l16 0z"/></svg>
				<input type="password" v-model="serverPassword" placeholder="Type your password">
			</div><br />

            <input type="checkbox" value="rememberMeValue" id="rememberMe"> <label for="rememberMe">Remember me</label><br /><br />

            <button class="submit" @click="connectToServer">Connect</button>
            <button class="secondary-submit" @click="demoMode">Demo Mode</button>
            <p v-if="errorMessage" class="error">{{ errorMessage }}</p>
        </div>
    </div>
</template>
<script setup lang="ts">
import { ref } from 'vue';
import { initiateServerConnection, initiateDemoConnection } from '../models/ServerConnection';

const serverIp = ref('');
const serverPassword = ref('');
const errorMessage = ref('');

async function connectToServer() {
    errorMessage.value = '';
    if (!serverIp.value || !serverPassword.value) {
        errorMessage.value = 'Please fill in all fields.';
        return;
    }

    try {
        const controller = new AbortController();
        const timeoutId = setTimeout(() => controller.abort(), 5000); // Prevents hanging

        const response = await fetch(`https://${serverIp.value}/test-connection?password=${serverPassword.value}`, {
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

        const data = await response.text();
        if (data === 'Connection successful') {
            // Handle successful connection
            initiateServerConnection(serverIp.value, serverPassword.value);
        } else {
            errorMessage.value = data || 'Connection failed.';
        }
    } catch (error) {
        errorMessage.value = (error as Error).message || 'An error occurred.';
    }
}

function demoMode() {
    initiateDemoConnection();
}

</script>
<style scoped>
.connect-container {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1200;
    backdrop-filter: blur(5px);
}

.connect-content {
    background: var(--background-dark-color);
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.1);
    width: 100%;
    max-width: 350px;
    margin: 20px;
}

.connect-content img {
    width: 50px;
    height: 50px;
    margin-bottom: 10px;
    text-align: center;
    display: block;
    margin-left: auto;
    margin-right: auto;
}

.connect-content h1 {
    text-align: center;
    margin-bottom: 20px;
    font-size: 24px;
    color: var(--primary-color);
}

.connect-content label {
    color: white;
    font-size: 12px;
    font-weight: bold;
}

.connect-content .input-container {
    width: 100%;
    box-sizing: border-box;
    border-bottom: 1px solid #ccc;
    display: flex;
    align-items: center;
    padding: 8px 0;
}
.connect-content svg {
    width: 20px;
    height: 20px;
    fill: #a7a7a7;
}
.connect-content .input-container input {
    width: calc(100% - 20px);
    padding: 10px;
    font-size: 14px;
    box-sizing: border-box;
    border: none;
    outline: none;
    color: white;
    background: rgba(0,0,0,0);
}
.connect-content .input-container input::placeholder {
    color: #a7a7a7;
    font-weight: 300;
}

.connect-content .submit {
    width: 100%;
    padding: 10px;
    background: var(--primary-color);
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 16px;
    cursor: pointer;
    transition: background 0.3s ease;
    margin-top: 10px;
    margin-bottom: 10px;
}

.connect-content .submit:hover {
    background: var(--primary-dark-color);
}

.connect-content .secondary-submit {
    width: 100%;
    padding: 10px;
    color: var(--primary-color);
    background: rgba(0,0,0,0);
    border: none;
    font-size: 16px;
    cursor: pointer;
    transition: color 0.3s ease;
    margin-top: 10px;
    margin-bottom: 10px;
}

.connect-content .secondary-submit:hover {
    color: var(--primary-dark-color);
}
</style>