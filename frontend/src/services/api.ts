import redaxios from 'redaxios';

interface EnvMeta extends ImportMeta {
  env: {
    SNOWPACK_PUBLIC_BASE_URL: string;
  };
}

const envMeta: EnvMeta = import.meta as EnvMeta;

export default redaxios.create({
  baseURL: envMeta.env.SNOWPACK_PUBLIC_BASE_URL,
  withCredentials: true,
}) as typeof redaxios;
