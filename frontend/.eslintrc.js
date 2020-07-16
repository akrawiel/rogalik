module.exports = {
  parserOptions: {
    ecmaVersion: 2019,
    sourceType: 'module'
  },
  parser: '@typescript-eslint/parser',
  env: {
    es6: true,
    browser: true
  },
  extends: 'standard-with-typescript',
  plugins: [
    'svelte3'
  ],
  overrides: [
    {
      files: ['*.svelte'],
      processor: 'svelte3/svelte3'
    }
  ],
  rules: {
    'import/first': 'off',
    'import/no-duplicates': 'off',
    'import/no-mutable-exports': 'off',
    'import/no-unresolved': 'off'
  }
}
