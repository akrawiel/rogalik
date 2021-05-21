import 'normalize.css';
import '@/styles/global.css';

import { SvelteComponent } from 'svelte/internal';

import App from './App.svelte';

const app: SvelteComponent = new App({
  target: document.querySelector('main'),
});

interface HmrMeta extends ImportMeta {
  hot: {
    accept: () => void;
    dispose: (disposeHandler: () => void) => void;
  }
}

const hmrMeta: HmrMeta = import.meta as HmrMeta;

if (hmrMeta.hot) {
  hmrMeta.hot.accept();

  hmrMeta.hot.dispose(() => {
    app.$destroy();
  });
}

export default app;
