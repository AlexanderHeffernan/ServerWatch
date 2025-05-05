import { ref } from 'vue';
import { notificationsManager } from './NotificationsManager';

class ServerConnection {
    private static instance: ServerConnection | null = null;
    private _address: string;
    private _password: string;
    private _serverName: string;
    private _metrics: any = null;
    private _tempConfig: any = null;
    private _demoMode = false;

    private constructor(address: string, password: string, serverName: string) {
        if (!address) { throw new Error('Address is required to establish a server connection.'); }
        if (!password) { throw new Error('Password is required to establish a server connection.'); }
        if (!serverName) { throw new Error('Server name is required to establish a server connection.'); }
        this._address = address;
        this._password = password;
        this._serverName = serverName;
    }

    public static async initiateConnection(address: string, password: string, serverName: string): Promise<ServerConnection> {
        if (!ServerConnection.instance) {
            ServerConnection.instance = new ServerConnection(address, password, serverName);
            
            await ServerConnection.instance.fetchMetrics();
            await ServerConnection.instance.fetchTempConfig();
            return ServerConnection.instance;
        } else {
            throw new Error('Connection already established. Use getInstance() to access the existing connection.');
        }
    }

    public static initiateDemoConnection(): ServerConnection {
        if (!ServerConnection.instance) {
            ServerConnection.instance = new ServerConnection('demo', 'demo', 'Demo Server');
            
            ServerConnection.instance._demoMode = true;
            ServerConnection.instance.fetchMetrics();
            ServerConnection.instance.fetchTempConfig();
            return ServerConnection.instance;
        } else {
            throw new Error('Demo connection already established. Use getInstance() to access the existing connection.');
        }
    }

    public static getInstance(): ServerConnection | null {
        if (!ServerConnection.instance) {
            return null;
        }
        return ServerConnection.instance;
    }

    public get address(): string { return this._address; }
    public get password(): string { return this._password; }
    public get serverName(): string { return this._serverName; }
    public get demoMode(): boolean { return this._demoMode; }

    public async refresh() {
        await Promise.all([
            this.fetchMetrics(), 
            this.fetchTempConfig()
        ]);
    }
    
    public getMetric(key: string): any {
        if (this._metrics && key in this._metrics) {
            return this._metrics[key];
        }
        return "N/A";
    }
    
    private async fetchMetrics() {
        if (this._demoMode) {
            const randomBytes = () => Math.floor(Math.random() * 1e12); // Generate a random number of bytes
            const randomMemoryTotal = randomBytes() / 75; // Generate a random number of bytes
            const randomDisk1Total = randomBytes(); // Generate a random number of bytes
            const randomDisk2Total = randomBytes(); // Generate a random number of bytes
            this._metrics = {
                "total_cpu_usage": Math.floor(Math.random() * 100),
                "individual_cpu_usage": Array.from({ length: 4 }, () => Math.floor(Math.random() * 100)),
                "memory_usage": randomMemoryTotal / Math.floor(Math.random() * 100),
                "memory_total": randomMemoryTotal,
                "cpu_temperature": Math.floor(Math.random() * 100),
                "individual_temperatures": Array.from({ length: 4 }, () => Math.floor(Math.random() * 100)),
                "disks": [
                    {
                        "label": "Disk 1",
                        "total": randomDisk1Total,
                        "used": randomDisk1Total / Math.floor(Math.random() * 100)
                    },
                    {
                        "label": "Disk 2",
                        "total": randomDisk2Total,
                        "used": randomDisk2Total / Math.floor(Math.random() * 100)
                    }
                ]
            };
            return;
        }

        try {
            const controller = new AbortController();
            const timeoutId = setTimeout(() => controller.abort(), 5000); // Prevents hanging

            const response = await fetch(`https://${this._address}/metrics?password=${this._password}`, {
                method: 'GET',
                signal: controller.signal,
                headers: {
                    "ngrok-skip-browser-warning": "true" // Bypass ngrok’s free tier warning
                }
            });

            clearTimeout(timeoutId);
            if (!response.ok) { throw new Error('Failed to fetch metrics from server.'); }
            this._metrics = await response.json();
        } catch (error) {
            this._metrics = null;
            if (error instanceof Error) {
                if (error.name === 'TypeError' && error.message.toLowerCase().includes('load failed')) {
                    notificationsManager.value?.addNotification('Failed to fetch metrics', 'Please try again soon. This may be caused by an excessive amount of requests in a short span of time.', 'error');
                }
            } else {
                console.error('Error fetching metrics:', error);
            }
        }
    }

    public getTempConfig(key: string): any {
        if (this._tempConfig && key in this._tempConfig) {
            return this._tempConfig[key];
        }
        return "N/A";
    }

    public async setTempConfig(warningTemp: number, shutdownTemp: number, warningsEnabled: boolean, shutdownEnabled: boolean) {
        if (this._demoMode) return;
        this._tempConfig = {
            "warning_temp": warningTemp,
            "shutdown_temp": shutdownTemp,
            "warnings_enabled": warningsEnabled,
            "shutdown_enabled": shutdownEnabled,
        };

        console.log('Setting temp config:', this._tempConfig);

        try {
            const controller = new AbortController();
            const timeoutId = setTimeout(() => controller.abort(), 5000); // Prevents hanging

            const response = await fetch(`https://${this._address}/set-temp-config?password=${this._password}&warning_temp=${warningTemp}&shutdown_temp=${shutdownTemp}&warnings_enabled=${warningsEnabled}&shutdown_enabled=${shutdownEnabled}`, {
                method: 'GET',
                signal: controller.signal,
                headers: {
                    "ngrok-skip-browser-warning": "true" // Bypass ngrok’s free tier warning
                }
            });

            clearTimeout(timeoutId);

            if (!response.ok) {
                throw new Error('Failed to set temp config on server.');
            }
        } catch (error) {
            console.error('Error setting temp config:', error);
        }
    }

    private async fetchTempConfig() {
        if (this._demoMode) {
            this._tempConfig = {
                "warning_temp": 75,
                "shutdown_temp": 85,
                "warnings_enabled": true,
                "shutdown_enabled": true,
            };
            return;
        }

        try {
            const controller = new AbortController();
            const timeoutId = setTimeout(() => controller.abort(), 5000); // Prevents hanging

            const response = await fetch(`https://${this._address}/get-temp-config?password=${this._password}`, {
                method: 'GET',
                signal: controller.signal,
                headers: {
                    "ngrok-skip-browser-warning": "true" // Bypass ngrok’s free tier warning
                }
            });

            clearTimeout(timeoutId);

            if (!response.ok) {
                throw new Error('Failed to fetch metrics from server.');
            }
            this._tempConfig = await response.json();
        } catch (error) {
            if (error instanceof Error) {
                if (error.name === 'TypeError' && error.message.toLowerCase().includes('load failed')) {
                    notificationsManager.value?.addNotification('Failed to fetch config file', 'Please try again soon. This may be caused by an excessive amount of requests in a short span of time.', 'error');
                }
            } else {
                console.error('Error fetching metrics:', error);
            }
            this._tempConfig = null;
        }
    }

    public async shutdown() {
        if (this._demoMode) return;
        try {
            const controller = new AbortController();
            const timeoutId = setTimeout(() => controller.abort(), 5000); // Prevents hanging

            const response = await fetch(`https://${this._address}/shutdown?password=${this._password}`, {
                method: 'GET',
                signal: controller.signal,
                headers: {
                    "ngrok-skip-browser-warning": "true" // Bypass ngrok’s free tier warning
                }
            });

            clearTimeout(timeoutId);

            if (!response.ok) {
                throw new Error('Failed to fetch metrics from server.');
            }
        } catch (error) {
            console.error('Error shutting down server:', error);
        }
    }

    public async reboot() {
        savedIp.value = this._address;
        savedPassword.value = this._password;
        savedDemoMode.value = this._demoMode;
        if (this._demoMode) { return; }    
        try {
            const controller = new AbortController();
            const timeoutId = setTimeout(() => controller.abort(), 5000); // Prevents hanging

            const response = await fetch(`https://${this._address}/reboot?password=${this._password}`, {
                method: 'GET',
                signal: controller.signal,
                headers: {
                    "ngrok-skip-browser-warning": "true" // Bypass ngrok’s free tier warning
                }
            });

            clearTimeout(timeoutId);

            if (!response.ok) {
                throw new Error('Failed to fetch metrics from server.');
            }
        } catch (error) {
            console.error('Error rebooting server:', error);
        }
    }

    public disconnect() {
        ServerConnection.instance = null;
    }
}

export const serverConnection = ref<ServerConnection | null>(ServerConnection.getInstance());
export const initiateServerConnection = async (address: string, password: string, serverName: string): Promise<void> => {
    try {
        serverConnection.value = await ServerConnection.initiateConnection(address, password, serverName);
        savedIp.value = null;
        savedPassword.value = null;
        savedDemoMode.value = false;
    } catch (error) {
        console.error('Error establishing server connection:', error);
        serverConnection.value = null;
    }
};

export function initiateDemoConnection(): void {
    try {
        serverConnection.value = ServerConnection.initiateDemoConnection();
        savedIp.value = null;
        savedPassword.value = null;
        savedDemoMode.value = false;
    } catch (error) {
        console.error('Error establishing demo connection:', error);
        serverConnection.value = null;
    }
}

export const savedIp = ref<string | null>(null);
export const savedPassword = ref<string | null>(null);
export const savedDemoMode = ref<boolean>(false);