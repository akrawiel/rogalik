<script lang="ts">
  import RoundButton from '@/atoms/RoundButton.svelte';
  import PlayIcon from '@/icons/PlayIcon.svelte';

  function log() {
    console.info('wow!');
  }
</script>

<div>Time tracking</div>
<RoundButton on:click={log} size="md">
  <PlayIcon />
</RoundButton>

<a href="/sign-out">Sign out</a>
