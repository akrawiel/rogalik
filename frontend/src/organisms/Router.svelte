<script lang="ts">
  import page from 'page';
  import { Route } from '@/config/routes';

  export let routes: Route[] = [];

  let currentPage: Route | null = null;

  const registerPath = ({ pathname }: { pathname: Route['path'] }) => {
    currentPage = routes.find(({ path }) => path === pathname) ?? null;
  };

  routes.forEach(({ path }) => {
    page(path, registerPath);
  });

  page();
</script>

<div class="router">
  {#if currentPage}
    {#await currentPage.component?.()}
      <div>Loading...</div>
    {:then pageComponent}
      <svelte:component this={pageComponent.default} />
    {/await}
  {/if}
</div>

<style lang="scss">
  .router {
    flex: 1;
  }
</style>
