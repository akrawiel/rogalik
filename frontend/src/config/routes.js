export default [
  {
    path: '/',
    component: () => import('@/pages/TimeTracking.svelte'),
  },
  {
    path: '/reports',
    component: () => import('@/pages/Reports.svelte'),
  },
  {
    path: '/settings',
    component: () => import('@/pages/Settings.svelte'),
  },
  {
    path: '/sign-in',
    auth: true,
    component: () => import('@/pages/SignIn.svelte'),
  },
  {
    path: '/sign-up',
    auth: true,
    component: () => import('@/pages/SignUp.svelte'),
  },
  {
    path: '/sign-out',
    auth: true,
    component: () => import('@/pages/SignOut.svelte'),
  },
];
