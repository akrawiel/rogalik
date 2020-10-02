<script>

</script>

<style lang="scss">
  .o-sideMenu {
    background-color: var(--primary);
    box-shadow: 0px 0px 4px var(--primary);
    box-sizing: border-box;
    color: var(--white);
    height: 100%;
    width: 256px;

    &__titleContainer {
      align-items: center;
      display: flex;
      padding: var(--spacing4) var(--spacing4);
      user-select: none;
    }

    &__titleLogo {
      border-radius: 40px;
      border: 1px solid var(--white);
      height: 40px;
      margin-right: var(--spacing2);
      width: 40px;
    }

    &__title {
      font-size: var(--fontSize5);
      font-variation-settings: var(--fontVariationExtraBold);
    }

    &__item {
      align-items: center;
      background-color: rgba(var(--secondarySpread), 0);
      box-sizing: border-box;
      color: var(--white);
      cursor: pointer;
      display: grid;
      grid-column-gap: var(--spacing2);
      grid-template-columns: var(--fontSize4) 1fr;
      padding: var(--spacing1) var(--spacing4);
      text-decoration: none;
      transition: var(--transitionNormal);
      user-select: none;
      width: 100%;

      &:hover {
        background-color: rgba(var(--secondarySpread), 0.5);
        font-variation-settings: var(--fontVariationBold);
      }

      > i {
        font-size: var(--fontSize4);
        line-height: 1;
        height: var(--fontSize4);
      }
    }
  }
</style>

<aside class="o-sideMenu">
  <div class="o-sideMenu__titleContainer">
    <div class="o-sideMenu__titleLogo" />
    <div class="o-sideMenu__title">Rogal</div>
  </div>
  <a class="o-sideMenu__item" href="/">
    <i class="mi-clock" />
    <span>Time tracking</span>
  </a>
  <a class="o-sideMenu__item" href="/reports">
    <i class="mi-clipboard-list" />
    <span>Reports</span>
  </a>
  <a class="o-sideMenu__item" href="/settings">
    <i class="mi-settings" />
    <span>Settings</span>
  </a>
</aside>
