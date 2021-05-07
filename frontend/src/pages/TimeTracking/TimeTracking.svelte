<script>
  import RoundButton from '@/atoms/RoundButton.svelte';
  import PlayIcon from '@/icons/PlayIcon.svelte';
</script>

<div>Time tracking</div>
<RoundButton>
  <PlayIcon />
</RoundButton>

<a href="/sign-out">Sign out</a>

<style>
  svg {
    width: 100px;
  }
</style>
