<script lang="ts">
  import page from 'page';

  import { signUp } from '@/store/auth';

  const form = {
    email: '',
    firstName: '',
    lastName: '',
    password: '',
  };

  function onSubmit() {
    signUp({ ...form }).finally(() => {
      page('/');
    });
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
    type="text"
    name="firstName"
    placeholder="First name"
    bind:value={form.firstName}
  />
  <input
    type="text"
    name="lastName"
    placeholder="Last name"
    bind:value={form.lastName}
  />
  <input
    type="password"
    name="password"
    placeholder="Password"
    bind:value={form.password}
  />
  <button type="submit">Sign up</button>
</form>

<a href="/sign-in">Sign in</a>
