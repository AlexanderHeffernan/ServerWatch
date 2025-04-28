import { ref, reactive } from 'vue';

class Notification {
    private _id: string;
    private _title: string;
    private _message: string;
    private _type: string;

    constructor(id: string, title: string, message: string, type: string) {
        this._id = id;
        this._title = title;
        this._message = message;
        this._type = type;
    }

    public get id(): string { return this._id; }
    public get title(): string { return this._title; }
    public get message(): string { return this._message; }
    public get type(): string { return this._type; }
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
        const notification = new Notification(id, title, message, type);
        this._notifications.push(notification);
    }

    private generateId(): string {
        return String(this._notifications.length);
    }

    public get notifications() {
        return this._notifications;
    }
}

export const notificationsManager = ref<NotificationsManager | null>(NotificationsManager.getInstance());