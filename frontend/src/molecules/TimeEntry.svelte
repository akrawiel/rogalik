<script>
  import AtomInput from "../atoms/Input.svelte";
  import AtomInputFrame from "../atoms/InputFrame.svelte";
  import MoleculeDateTimeInput from "../molecules/DateTimeInput.svelte";
</script>

<style lang="scss">
  .m-timeEntry {
    border: 1px solid var(--primary);
    border-radius: 8px;
    display: grid;
    grid-column-gap: 0.5rem;
    grid-template-columns: 1fr 10ch 10ch;
    padding: var(--spacing2);
  }
</style>

<div class="m-timeEntry">
  <AtomInputFrame>
    <AtomInput value="" placeholder="Task description" />
  </AtomInputFrame>
  <MoleculeDateTimeInput value="" />
  <MoleculeDateTimeInput value="" />
</div>
