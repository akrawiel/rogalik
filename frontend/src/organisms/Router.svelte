<script>
  import page from 'page';

  export let paths = [];

  let currentPage = null;

  const registerPath = ({ pathname }) => {
    currentPage = paths.find(({ path }) => path === pathname) ?? null;
  };

  paths.forEach(({ path }) => {
    page(path, registerPath);
  });

  page();
</script>

<style lang="scss">
  .o-router {
    flex: 1;
  }
</style>

<div class="o-router">
  {#if currentPage}
    {#await currentPage.component()}
      <div>Loading...</div>
    {:then pageComponent}
      <svelte:component this={pageComponent.default} />
    {/await}
  {/if}
</div>
