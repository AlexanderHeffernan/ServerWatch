import { afterEach, expect, test, vi } from 'vitest';
import { requestServer } from './api';
import { initiateServerConnection, serverConnection, savedIp, savedPassword } from './ServerConnection';
import { notificationsManager } from './NotificationsManager';

afterEach(() => {
    serverConnection.value?.disconnect();
    serverConnection.value = null;
    savedIp.value = null;
    savedPassword.value = null;
    notificationsManager.value?.clearNotifications();
    vi.unstubAllGlobals();
    vi.restoreAllMocks();
    vi.useRealTimers();
});

test('connection, metrics, settings and power use header credentials and correct methods', async () => {
    const requests: Array<{ url: string; options: RequestInit }> = [];
    vi.stubGlobal('fetch', vi.fn(async (url: string, options: RequestInit) => {
        requests.push({ url, options });
        return Response.json({ warning_temp: 75, shutdown_temp: 85, total_cpu_usage: 13 });
    }));
    const password = 'secret &?#%+';
    await requestServer('SERVER.example:443', password, '/test-connection');
    await initiateServerConnection('server.example:49160', password, 'Test server');
    const connection = serverConnection.value!;
    expect(connection.getMetric('total_cpu_usage')).toBe(13);
    await connection.setTempConfig(62, 93, false, true);
    expect(connection.getTempConfig('warning_temp')).toBe(62);
    expect(await connection.shutdown()).toBe(true);
    expect(await connection.reboot()).toBe(true);
    expect(savedPassword.value).toBe(password);
    expect(requests.map(({ url, options }) => [new URL(url).pathname, options.method])).toEqual([
        ['/test-connection', 'GET'], ['/metrics', 'GET'], ['/get-temp-config', 'GET'],
        ['/set-temp-config', 'PUT'], ['/shutdown', 'POST'], ['/reboot', 'POST'],
    ]);
    for (const { url, options } of requests) {
        expect(new URL(url).search).toBe('');
        expect(url).not.toContain(password);
        expect(options.headers).toMatchObject({ Authorization: `Bearer ${password}` });
        expect(options.redirect).toBe('error');
        expect(options.credentials).toBe('same-origin');
    }
    expect(JSON.parse(requests[3].options.body as string)).toEqual({ warning_temp: 62, shutdown_temp: 93, warnings_enabled: false, shutdown_enabled: true });
});

test('denied writes do not claim settings or power changes or schedule reconnect', async () => {
    vi.spyOn(console, 'error').mockImplementation(() => undefined);
    vi.stubGlobal('fetch', vi.fn(async () => Response.json({ warning_temp: 75 })));
    await initiateServerConnection('server.example', 'reader', 'Test server');
    vi.stubGlobal('fetch', vi.fn(async () => new Response('Forbidden', { status: 403 })));
    const connection = serverConnection.value!;
    await connection.setTempConfig(60, 90, true, true);
    expect(connection.getTempConfig('warning_temp')).toBe(75);
    expect(await connection.shutdown()).toBe(false);
    expect(await connection.reboot()).toBe(false);
    expect(savedIp.value).toBeNull();
    expect(savedPassword.value).toBeNull();
    expect(notificationsManager.value?.notifications).toHaveLength(3);
});

test('addresses cannot smuggle URL credentials or query parameters', async () => {
    const fetch = vi.fn();
    vi.stubGlobal('fetch', fetch);
    for (const address of ['user@server.example', 'server.example/path', 'server.example?password=x', 'server.example#x', 'server.example\\evil']) {
        await expect(requestServer(address, 'secret', '/metrics')).rejects.toThrow();
    }
    expect(fetch).not.toHaveBeenCalled();
});

test('network failure clears timeout and hanging requests abort after five seconds', async () => {
    vi.useFakeTimers();
    vi.stubGlobal('fetch', vi.fn(async () => { throw new Error('Network failure'); }));
    await expect(requestServer('server.example', 'secret', '/metrics')).rejects.toThrow('Network failure');
    expect(vi.getTimerCount()).toBe(0);
    vi.stubGlobal('fetch', vi.fn((_url, options: RequestInit) => new Promise((_resolve, reject) => {
        options.signal?.addEventListener('abort', () => reject(new Error('Aborted')));
    })));
    const pending = expect(requestServer('server.example', 'secret', '/metrics')).rejects.toThrow('Aborted');
    await vi.advanceTimersByTimeAsync(5000);
    await pending;
    expect(vi.getTimerCount()).toBe(0);
});
