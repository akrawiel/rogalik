import { SvelteComponentDev } from 'svelte/internal';

export interface Route {
  path: string;
  required?: string[];
  component: () => Promise<{ default: typeof SvelteComponentDev }>;
}

const routes: Route[] = [
  {
    path: '/',
    component: () => import('@/pages/TimeTracking/TimeTracking.svelte'),
    required: ['auth'],
  },
  {
    path: '/reports',
    component: () => import('@/pages/Reports.svelte'),
    required: ['auth'],
  },
  {
    path: '/settings',
    component: () => import('@/pages/Settings.svelte'),
    required: ['auth'],
  },
  {
    path: '/sign-in',
    component: () => import('@/pages/SignIn.svelte'),
  },
  {
    path: '/sign-up',
    component: () => import('@/pages/SignUp.svelte'),
  },
  {
    path: '/sign-out',
    component: () => import('@/pages/SignOut.svelte'),
  },
];

export default routes;
