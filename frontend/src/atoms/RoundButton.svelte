<script lang="ts">
  export let size: 'md' | 'lg' | 'xl' | 'xxl' = 'md';
</script>

<button on:click|preventDefault class={size} on:contextmenu|preventDefault>
  <slot />
</button>

<style>
  button {
    @apply bg-primary active:bg-primary-dark border-0 cursor-pointer rounded-full
      p-2 outline-none focus:outline-none shadow-md select-none;
  }

  button.md {
    @apply w-12 h-12;
  }

  button.lg {
    @apply w-18 h-18;
  }

  button.xl {
    @apply w-24 h-24;
  }

  button.xxl {
    @apply w-32 h-32;
  }
</style>
