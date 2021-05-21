<script lang="ts">
  import page from 'page';
  import { onMount } from 'svelte';

  import routes from '@/config/routes';
  import Router from '@/organisms/Router.svelte';
  import { fetchProfile } from '@/store/auth';

  onMount(() => {
    const currentPageRegex = new RegExp(`^${window.location.pathname}$`);

    const currentPage = routes.find(({ path }) => currentPageRegex.test(path));

    const currentPageRequiresAuth =
      currentPage?.required?.includes('auth') ?? false;

    fetchProfile()
      .then(() => {
        if (!currentPageRequiresAuth) {
          page('/');
        }
      })
      .catch(() => {
        if (currentPageRequiresAuth) {
          page('/sign-in');
        }
      });
  });
</script>

<Router {routes} />

<style windi:preflights:global windi:safelist:global>
</style>
