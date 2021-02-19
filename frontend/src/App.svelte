<script>
  import page from 'page';
  import { onMount } from 'svelte';

  import Router from '@/organisms/Router.svelte';
  import auth, { initializeAuth } from '@/store/auth';

  const paths = [
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

  onMount(() => {
    initializeAuth().then(() => {
      const currentPageRegex = new RegExp(`^${window.location.pathname}$`);

      const currentPage = paths.find(({ path }) => currentPageRegex.test(path));

      if (!$auth.jwt && !currentPage?.auth) {
        page('/sign-in');
      }
    });
  });

  $: ready = $auth.ready;
</script>

{#if ready}
  <Router {paths} />
{:else}
  <div>Loading...</div>
{/if}
