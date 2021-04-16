<script>
  import page from 'page';

  import { signUp } from '@/store/auth';

  const form = {
    email: '',
    username: '',
    password: '',
  };

  function onSubmit() {
    signUp({ ...form }).then(() => {
      page('/');
    });
  }
</script>

<form on:submit|preventDefault={onSubmit}>
  <input
    type="email"
    name="username"
    placeholder="Username"
    bind:value={form.email} />
  <input
    type="text"
    name="username"
    placeholder="Username"
    bind:value={form.username} />
  <input
    type="password"
    name="password"
    placeholder="Password"
    bind:value={form.password} />
  <button type="submit">Sign up</button>
</form>

<a href="/sign-in">Sign in</a>
