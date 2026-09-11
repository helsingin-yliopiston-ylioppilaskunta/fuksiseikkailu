import { client } from './generated/client.gen';

client.setConfig({
    baseUrl: import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:3000',
});

client.interceptors.request.use((request: Request) => {
    const token = localStorage.getItem('access_token');
    if (token) {
        request.headers.set('Authorization', `Bearer ${token}`);
    }
    return request;
});
