<script>
  import page from 'page';
  import { onMount } from 'svelte';

  import routes from '@/config/routes';
  import Router from '@/organisms/Router.svelte';
  import auth, { initializeAuth } from '@/store/auth';

  onMount(() => {
    initializeAuth().then(() => {
      const currentPageRegex = new RegExp(`^${window.location.pathname}$`);

      const currentPage = routes.find(({ path }) =>
        currentPageRegex.test(path)
      );

      if (!$auth.jwt && !currentPage?.auth) {
        page('/sign-in');
      }
    });
  });

  $: ready = $auth.ready;
</script>

{#if ready}
  <Router {routes} />
{:else}
  <div>Loading...</div>
{/if}
