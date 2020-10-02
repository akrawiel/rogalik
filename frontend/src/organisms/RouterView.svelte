<script>
  import page from "page";

  const paths = [
    {
      path: "/",
      component: () => import("../pages/TimeTracking.svelte"),
    },
    {
      path: "/reports",
      component: () => import("../pages/Reports.svelte"),
    },
    {
      path: "/settings",
      component: () => import("../pages/Settings.svelte"),
    },
  ];

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
  .o-routerView {
    flex: 1;
  }
</style>

<div class="o-routerView">
  {#if currentPage}
    {#await currentPage.component()}
      <div>Loading...</div>
    {:then pageComponent}
      <svelte:component this={pageComponent.default} />
    {/await}
  {/if}
</div>
