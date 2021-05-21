<script lang="ts">
  import page from 'page';

  import { fetchProfile, signIn } from '@/store/auth';

  const form = {
    email: '',
    password: '',
  };

  async function onSubmit() {
    await signIn({ ...form });

    await fetchProfile();

    page('/');
  }
</script>

<form on:submit|preventDefault={onSubmit}>
  <input
    type="email"
    name="email"
    placeholder="E-mail address"
    bind:value={form.email}
  />
  <input
    type="password"
    name="password"
    placeholder="Password"
    bind:value={form.password}
  />
  <button type="submit">Sign in</button>
</form>

<a href="/sign-up">Sign up</a>
