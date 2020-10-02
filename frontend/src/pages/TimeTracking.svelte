<script>
  import MoleculeTimeEntry from "../molecules/TimeEntry.svelte";

  let value = "";

  const handleInput = (event) => {
    value = event.target.value;
  };
</script>

<div>
  <MoleculeTimeEntry {value} on:change={handleInput} />
</div>
