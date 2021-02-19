import redaxios from 'redaxios';

export const api = redaxios.create({
  baseURL: import.meta.env.SNOWPACK_PUBLIC_BASE_URL,
  withCredentials: true,
});

export const publicApi = redaxios.create({
  baseURL: import.meta.env.SNOWPACK_PUBLIC_BASE_URL,
});
