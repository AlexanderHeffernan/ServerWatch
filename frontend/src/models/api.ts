// Keep connection, reconnect and dashboard requests on the same wire contract.
export async function requestServer(address: string, password: string, path: string, method = 'GET', body?: unknown): Promise<Response> {
    const url = new URL(`https://${address}`);
    if (/[\s/\\?#@]/.test(address) || url.username || url.password || url.pathname !== '/' || url.search || url.hash) {
        throw new Error('Enter a hostname or IP address with an optional port, not a URL.');
    }
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), 5000);
    try {
        return await fetch(`${url.origin}${path}`, {
            method,
            headers: {
                Authorization: `Bearer ${password}`,
                'ngrok-skip-browser-warning': 'true',
                ...(body === undefined ? {} : { 'Content-Type': 'application/json' }),
            },
            body: body === undefined ? undefined : JSON.stringify(body),
            signal: controller.signal,
            credentials: 'same-origin',
            redirect: 'error',
            cache: 'no-store',
        });
    } finally {
        clearTimeout(timeout);
    }
}
