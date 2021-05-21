module.exports = {
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 2020,
    sourceType: 'module',
    extraFileExtensions: ['.svelte'],
    project: './tsconfig.json',
    tsconfigRootDir: __dirname
  },
  extends: [
    'airbnb-typescript/base',
    'plugin:@typescript-eslint/recommended',
    'plugin:@typescript-eslint/recommended-requiring-type-checking',
    'plugin:prettier/recommended'
  ],
  env: {
    es6: true,
    browser: true,
    node: true
  },
  plugins: [
    'svelte3',
  ],
  overrides: [
    {
      files: ['**/*.svelte'],
      processor: 'svelte3/svelte3',
      rules: {
        'import/first': 'off',
        'import/no-duplicates': 'off',
        'import/no-mutable-exports': 'off',
        'import/no-unresolved': 'off',
        'import/prefer-default-export': 'off',
        'prettier/prettier': 'off'
      },
    },
  ],
  settings: {
    'import/resolver': {
      alias: {
        map: [
          ['@', './src'],
        ],
        extensions: ['.js', '.svelte', '.json'],
      },
    },
    'svelte3/typescript': true,
    'svelte3/ignore-styles': () => true
  },
};
