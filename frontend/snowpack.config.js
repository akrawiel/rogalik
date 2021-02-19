/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  mount: {
    public: '/',
    src: '/dist',
  },
  plugins: [
    '@snowpack/plugin-svelte',
    '@snowpack/plugin-dotenv',
    ['@snowpack/plugin-run-script', {
      cmd: 'eslint src --ext .js,jsx,.ts,.tsx,.svelte',
      watch: 'esw -w --clear src --ext .js,jsx,.ts,.tsx,.svelte',
    }],
  ],
  routes: [
    {
      match: 'routes',
      src: '.*',
      dest: '/index.html',
    },
  ],
  optimize: {
    bundle: true,
    minify: true,
    target: 'es2017',
  },
  alias: {
    '@': './src',
  },
  packageOptions: {
    /* ... */
  },
  devOptions: {
    port: 8000,
    open: 'none',
  },
  buildOptions: {
    /* ... */
  },
};
