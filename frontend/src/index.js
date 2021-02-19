import 'normalize.css';

import App from './App.svelte';

const app = new App({
  target: document.querySelector('main'),
});

if (import.meta.hot) {
  import.meta.hot.accept();

  import.meta.hot.dispose(() => {
    app.$destroy();
  });
}

export default app;
