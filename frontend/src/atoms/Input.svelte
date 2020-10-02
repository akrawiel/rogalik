<script>
  export let type = "text";
  export let placeholder = "";
  export let value;
</script>

<style lang="scss">
  .a-input {
    background: transparent;
    border: 0;
    caret-color: var(--white);
    color: var(--white);
    font-size: var(--fontSize3);
    line-height: normal;
    height: 20px;
    padding: 0;
    text-align: inherit;
    width: 100%;

    &::placeholder {
      color: var(--white);
      opacity: 0.7;
    }
  }
</style>

<input
  class="a-input"
  {type}
  {value}
  {placeholder}
  on:input
  on:focus
  on:blur
  on:keydown />
