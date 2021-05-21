/** @type {import("snowpack").SnowpackUserConfig } */

module.exports = {
  mount: {
    public: '/',
    src: '/dist',
  },
  plugins: [
    ['@snowpack/plugin-svelte', {
      preprocess: [
        require('svelte-windicss-preprocess').preprocess(),
        require('svelte-preprocess')(),
      ],
    }],
    '@snowpack/plugin-typescript',
    '@snowpack/plugin-dotenv',
    '@snowpack/plugin-postcss',
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
