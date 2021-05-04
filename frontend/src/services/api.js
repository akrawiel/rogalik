import redaxios from 'redaxios';

export default redaxios.create({
  baseURL: import.meta.env.SNOWPACK_PUBLIC_BASE_URL,
  withCredentials: true,
});
