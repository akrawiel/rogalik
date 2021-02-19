<script>
  import page from 'page';

  import { signIn } from '@/store/auth';

  let form = {
    username: '',
    password: '',
  };

  function onSubmit() {
    signIn({ ...form }).then(() => {
      page('/');
    });
  }
</script>

<form on:submit|preventDefault={onSubmit}>
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
  <button type="submit">Sign in</button>
</form>

<a href="/sign-up">Sign up</a>
