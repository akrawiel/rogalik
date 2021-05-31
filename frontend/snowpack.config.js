/** @type {import("snowpack").SnowpackUserConfig } */

const { preprocess: windi } = require('svelte-windicss-preprocess')
const sveltePreprocess = require('svelte-preprocess')

module.exports = {
  mount: {
    public: '/',
    src: '/dist',
  },
  plugins: [
    ['@snowpack/plugin-svelte', {
      preprocess: [
        sveltePreprocess({}),
        windi({})
      ],
    }],
    '@snowpack/plugin-typescript',
    '@snowpack/plugin-dotenv',
    ['@snowpack/plugin-sass', {
      native: true,
    }],
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
