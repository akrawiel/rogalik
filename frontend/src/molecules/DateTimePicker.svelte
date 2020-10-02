<script>
  import MoleculeCalendar from "../molecules/Calendar.svelte";
</script>

<style lang="scss">
  .m-dateTimePicker {
    border-radius: 8px;
    box-shadow: 0 4px 8px rgba(var(--blackSpread), 0.35);
    display: inline-block;
    left: 50%;
    position: absolute;
    transform: translateX(-50%);
    top: calc(100% + var(--spacing1));
  }
</style>

<div class="m-dateTimePicker">
  <MoleculeCalendar />
</div>
