<script>
  import AtomInput from "../atoms/Input.svelte";
  import AtomInputFrame from "../atoms/InputFrame.svelte";
  import MoleculeDateTimePicker from "./DateTimePicker.svelte";

  export let value;

  let calendarShown = false;

  const showCalendar = () => {
    calendarShown = true;
  };

  const hideCalendar = () => {
    calendarShown = false;
  };

  const handleInput = (event) => {
    event.preventDefault();
  };
</script>

<style lang="scss">
  .m-dateTimeInput {
    position: relative;
    text-align: center;
  }
</style>

<div class="m-dateTimeInput">
  <AtomInputFrame>
    <AtomInput
      on:focus={showCalendar}
      on:blur={hideCalendar}
      on:input={handleInput}
      {value} />
  </AtomInputFrame>
  {#if calendarShown}
    <MoleculeDateTimePicker />
  {/if}
</div>
