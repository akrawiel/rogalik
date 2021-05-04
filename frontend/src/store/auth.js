import { createEffect, createStore } from 'effector';

import api from '@/services/api';

export const fetchProfile = createEffect(async () => {
  const response = await api.get('/auth/me');

  return response.data;
});

export const signIn = createEffect(async ({ email, password }) => {
  const response = await api.post('/auth/sign-in', { email, password });

  return response.data;
});

export const signOut = createEffect(async () => {
  const response = await api.get('/auth/sign-out');

  return response.data;
});

export const signUp = createEffect(async ({
  firstName, lastName, email, password,
}) => {
  const response = await api.post('/auth/sign-up', {
    email,
    first_name: firstName,
    last_name: lastName,
    password,
  });

  return response.data;
});

function createAuth() {
  const store = createStore({
    user: null,
    error: null,
  });

  store.on([fetchProfile.doneData, signUp.doneData], (state, user) => ({
    ...state,
    user,
    error: null,
  }));

  store.on(fetchProfile.failData, (state, error) => ({
    ...state,
    user: null,
    error,
  }));

  store.on(signOut.doneData, (state) => ({
    ...state,
    user: null,
    error: null,
  }));

  return store;
}

export default createAuth();
