<style lang="scss">
  .a-inputFrame {
    align-items: center;
    background-color: var(--secondary);
    border: 1px solid var(--secondary);
    border-radius: 4px;
    box-shadow: inset 0 0 8px rgba(var(--primarySpread), 0);
    color: var(--white);
    display: flex;
    flex-flow: row nowrap;
    padding: var(--spacing2);
    transition: var(--transitionNormal);

    &:focus-within {
      border: 1px solid var(--primary);
      box-shadow: inset 0 0 8px rgba(var(--primarySpread), 1);
    }
  }
</style>

<div class="a-inputFrame">
  <slot />
</div>
