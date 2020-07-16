module.exports = {
  plugins: {
    autoprefixer: {},
    cssnano: {},
    '@fullhuman/postcss-purgecss': {
      content: ['./src/**/*.svelte']
    }
  }
}
