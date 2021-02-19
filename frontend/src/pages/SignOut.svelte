<script>
  import page from 'page';
  import { onMount } from 'svelte';

  import { signOut } from '@/store/auth';

  onMount(() => {
    signOut();
    page('/sign-in');
  });
</script>
