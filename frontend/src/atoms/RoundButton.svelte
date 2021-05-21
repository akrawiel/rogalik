<script lang="ts">
  export let size: 'md' | 'lg' | 'xl' = 'md';
</script>

<button on:click class={size}>
  <slot />
</button>

<style>
  button {
    @apply bg-primary active:bg-primary-dark border-0 cursor-pointer rounded-full
      p-2 outline-none focus:outline-none shadow-md;
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
</style>
