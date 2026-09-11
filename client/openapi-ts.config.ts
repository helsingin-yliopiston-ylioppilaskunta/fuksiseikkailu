import { defineConfig } from '@hey-api/openapi-ts';

export default defineConfig({
    client: '@hey-api/client-fetch',
    input: 'http://127.0.0.1:3000/api-docs/openapi.json',
    output: 'src/api/generated',
    plugins: [
        '@tanstack/react-query',
    ],
});
