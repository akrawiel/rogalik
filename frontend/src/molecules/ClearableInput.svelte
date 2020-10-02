<script>
  import { createEventDispatcher } from "svelte";

  import AtomInput from "../atoms/Input.svelte";
  import AtomInputFrame from "../atoms/InputFrame.svelte";

  const dispatch = createEventDispatcher();

  export let value;

  const clearValue = () => {
    dispatch("clear");
  };
</script>

<style lang="scss">
  .m-clearableInput {
    &__clearButton {
      cursor: pointer;
      color: currentColor;
      font-size: var(--fontSize3);

      > i {
        align-items: center;
        height: 100%;
        display: flex;
      }
    }
  }
</style>

<AtomInputFrame>
  <AtomInput type="text" on:input {value} />
  <div on:click={clearValue} class="m-clearableInput__clearButton">
    <i class="mi-close" />
  </div>
</AtomInputFrame>
