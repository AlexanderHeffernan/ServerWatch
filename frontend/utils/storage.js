// Securely manage localStorage for credentials
export function loadCredentials() {
    return {
        ipAddress: localStorage.getItem("ipAddress") || "",
        password: localStorage.getItem("password") || "",
        rememberMe: localStorage.getItem("ipAddress" && localStorage.getItem("password"))
    };
}

export function saveCredentials(ipAddress, password, rememberMe) {
    if (rememberMe) {
        localStorage.setItem("ipAddress", ipAddress);
        localStorage.setItem("password", password);
    } else {
        localStorage.removeItem("ipAddress");
        localStorage.removeItem("password");
    }
}