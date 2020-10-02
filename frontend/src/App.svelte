<script>
  import OrganismRouterView from "./organisms/RouterView.svelte";
  import OrganismSideMenu from "./organisms/SideMenu.svelte";
</script>

<style lang="scss">
  main {
    display: flex;
    height: 100%;
    width: 100%;
  }
</style>

<main>
  <OrganismRouterView />
  <OrganismSideMenu />
</main>
