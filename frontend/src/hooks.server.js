// src/hooks.server.js
export const handle = async ({ event, resolve }) => {
  if (event.url.pathname.startsWith('/api')) {
    const backendUrl = process.env.BACKEND_URL || 'http://172.18.0.1:3000';
    const apiPath = event.url.pathname;
    const query = event.url.search || '';

    try {
      const response = await fetch(`${backendUrl}${apiPath}${query}`, {
        method: event.request.method,
        headers: event.request.headers,
        body: event.request.body
      });

      return new Response(await response.text(), {
        status: response.status,
        headers: response.headers
      });
    } catch (err) {
      console.error('API proxy error:', err);
      return new Response(JSON.stringify({ error: 'Backend unavailable' }), {
        status: 502
      });
    }
  }

  return resolve(event);
}