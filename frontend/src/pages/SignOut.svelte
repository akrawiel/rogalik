<script lang="ts">
  import page from 'page';
  import { onMount } from 'svelte';

  import { signOut } from '@/store/auth';

  onMount(() => {
    signOut().finally(() => {
      page('/sign-in');
    });
  });
</script>
