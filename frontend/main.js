import { loadCredentials, saveCredentials } from "./utils/storage.js";
import { fetchMetrics } from "./api/fetchMetrics.js";
import { updateUI } from "./ui/updater.js";

// State
let currentIP = "";
let currentPassword = "";
let demoMode = false;

// DOM Elements
const ipInput = document.getElementById("ipAddress");
const passwordInput = document.getElementById("password");
const rememberMeCheck = document.getElementById("rememberMe");
const connectPopup = document.getElementById("connect-popup");

// Initialize from localStorage
function initialize() {
    const { ipAddress, password, rememberMe } = loadCredentials();
    ipInput.value = ipAddress;
    passwordInput.value = password;
    rememberMeCheck.checked = rememberMe;
}

async function handleFetchMetrics() {
    if (demoMode) { updateUI(await fetchMetrics("", "", demoMode)); return; }
    const ip = ipInput.value.trim();
    const password = passwordInput.value.trim();

    if (!ip || !password) {
        alert("IP address and password are required.");
        return;
    }

    currentIP = ip;
    currentPassword = password;

    try {
        const data = await fetchMetrics(ip, password, demoMode);
        console.log("Data: ", data);
        updateUI(data);
        saveCredentials(ip, password, rememberMeCheck.checked);
        connectPopup.classList.add("hidden");
    } catch (error) {
        alert(`Failed to fetch metrics: ${error.message}`);
    }
}

async function handleRefreshMetrics() {
    if (!(currentIP && currentPassword) && !demoMode) return;
    try {
        const data = await fetchMetrics(currentIP, currentPassword, demoMode);
        updateUI(data);
    } catch (error) {
        alert(`Failed to refresh metrics: ${error.message}`);
    }
}

function handleDemoMode() {
    demoMode = true;
    connectPopup.classList.add("hidden");
    handleFetchMetrics();
}

function flipCard(cardId) {
    document.querySelector(`#${cardId} .card-flip`).classList.toggle("flipped");
}

document.addEventListener("DOMContentLoaded", () => {
    // Event Listeners
    const fetchMetricsBtn = document.getElementById("fetchMetricsBtn");
    const refreshMetricsBtn = document.getElementById("refreshMetricsBtn");
    const tryDemoBtn = document.getElementById("tryDemoBtn");

    if (fetchMetricsBtn) fetchMetricsBtn.addEventListener("click", handleFetchMetrics);
    else console.error("Element with ID 'fetchMetricsBtn' not found");

    if (refreshMetricsBtn) refreshMetricsBtn.addEventListener("click", handleRefreshMetrics);
    else console.error("Element with ID 'refreshMetricsBtn' not found");

    if (tryDemoBtn) tryDemoBtn.addEventListener("click", handleDemoMode);
    else console.error("Element with ID 'tryDemoBtn' not found");

    const flipCards = document.querySelectorAll(".flip-card");
    flipCards.forEach(card => card.addEventListener("click", () => flipCard(card.id)));

    // Initialize app
    initialize();
});