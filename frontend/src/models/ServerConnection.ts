import { ref } from 'vue';

class ServerConnection {
    private static instance: ServerConnection;
    private _address: string;
    private _password: string;
    private _metrics: any = null;

    private constructor(address: string, password: string) {
        if (!address) { throw new Error('Address is required to establish a server connection.'); }
        if (!password) { throw new Error('Password is required to establish a server connection.'); }
        this._address = address;
        this._password = password;
    }

    public static async initiateConnection(address: string, password: string): Promise<ServerConnection> {
        if (!ServerConnection.instance) {
            ServerConnection.instance = new ServerConnection(address, password);
            await ServerConnection.instance.fetchMetrics();
            return ServerConnection.instance;
        } else {
            throw new Error('Connection already established. Use getInstance() to access the existing connection.');
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
    
    public getMetric(key: string): string | number {
        if (this._metrics && key in this._metrics) {
            return this._metrics[key];
        }
        return "N/A";
    }
    
    private async fetchMetrics() {
        try {
            const response = await fetch(`https://${this._address}/metrics?password=${this._password}`);
            if (!response.ok) {
                throw new Error('Failed to fetch metrics from server.');
            }
            this._metrics = await response.json();
        } catch (error) {
            console.error('Error fetching metrics:', error);
            this._metrics = null;
        }
    }
}

export const serverConnection = ref<ServerConnection | null>(ServerConnection.getInstance());
export const initiateServerConnection = async (address: string, password: string): Promise<void> => {
    try {
        serverConnection.value = await ServerConnection.initiateConnection(address, password);
    } catch (error) {
        console.error('Error establishing server connection:', error);
        serverConnection.value = null;
    }
};