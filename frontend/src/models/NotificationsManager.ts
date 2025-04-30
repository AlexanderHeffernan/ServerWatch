import { ref, reactive } from 'vue';

class Notification {
    private _id: string;
    private _title: string;
    private _message: string;
    private _type: string;
    private _timestamp: Date;

    constructor(id: string, title: string, message: string, type: string, timestamp: Date) {
        this._id = id;
        this._title = title;
        this._message = message;
        this._type = type;
        this._timestamp = timestamp;
    }

    public get id(): string { return this._id; }
    public get title(): string { return this._title; }
    public get message(): string { return this._message; }
    public get type(): string { return this._type; }
    public get timestamp(): Date { return this._timestamp; }
    public get formattedTimestamp(): string {
        return this._timestamp.toLocaleString('en-NZ', {
            hour: '2-digit',
            minute: '2-digit'
        });
    }
    public get formattedDate(): string {
        return this._timestamp.toLocaleString('en-NZ', {
            year: 'numeric',
            month: '2-digit',
            day: '2-digit'
        });
    }
}

class NotificationsManager {
    private static instance: NotificationsManager;
    private _notifications = reactive<Notification[]>([]);

    private constructor() {
        return;
    }
    public static getInstance(): NotificationsManager {
        if (!NotificationsManager.instance) {
            NotificationsManager.instance = new NotificationsManager();
        }
        return NotificationsManager.instance;
    }

    public addNotification(title: string, message: string, type: string): void {
        const id = this.generateId();
        const timestamp = new Date();
        const notification = new Notification(id, title, message, type, timestamp);
        this._notifications.push(notification);
    }

    public removeNotification(id: string): void {
        const index = this._notifications.findIndex(notification => notification.id === id);
        if (index !== -1) {
            this._notifications.splice(index, 1);
        }
    }

    private generateId(): string {
        return String(this._notifications.length);
    }

    public get notifications() {
        return this._notifications;
    }
}

export const notificationsManager = ref<NotificationsManager | null>(NotificationsManager.getInstance());