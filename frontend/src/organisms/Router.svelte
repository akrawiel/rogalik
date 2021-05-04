<script>
  import page from 'page';

  export let routes = [];

  let currentPage = null;

  const registerPath = ({ pathname }) => {
    currentPage = routes.find(({ path }) => path === pathname) ?? null;
  };

  routes.forEach(({ path }) => {
    page(path, registerPath);
  });

  page();
</script>

<style lang="scss">
  .router {
    flex: 1;
  }
</style>

<div class="router">
  {#if currentPage}
    {#await currentPage.component()}
      <div>Loading...</div>
    {:then pageComponent}
      <svelte:component this={pageComponent.default} />
    {/await}
  {/if}
</div>
