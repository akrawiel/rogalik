<script lang="ts">
  export let name: string;
</script>

<style global lang="sass">
  html,
  body
    font-family: 'Varta', sans-serif

  h1
    color: #ff3e00
</style>

<main>
  <h1>Hello {name}!</h1>
  <p>
    Visit the
    <a href="https://svelte.dev/tutorial">Svelte tutorial</a>
    to learn how to build Svelte apps.
  </p>
  <button>test</button>
</main>

