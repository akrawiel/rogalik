import { createEffect, createEvent, createStore } from 'effector';
import localforage from 'localforage';

import { publicApi } from '@/services/api';

export const initializeAuth = createEffect(async () => {
  const [jwt, user] = await Promise.all([
    localforage.getItem('jwt'),
    localforage.getItem('user'),
  ]);

  return {
    jwt, user,
  };
});

export const signIn = createEffect(async ({ username, password }) => {
  const response = await publicApi.post('/auth/local', { identifier: username, password });

  return response.data;
});

export const signOut = createEvent();

export const signUp = createEffect(async ({ username, email, password }) => {
  const response = await publicApi.post('/auth/local/register', { username, email, password });

  return response.data;
});

function createAuth() {
  const store = createStore({
    jwt: null,
    user: null,
    ready: false,
  });

  store.on(initializeAuth.doneData, (_, { jwt, user }) => ({
    jwt,
    user,
    ready: true,
  }));

  store.on([signIn.doneData, signUp.doneData], (state, { jwt, user }) => ({
    ...state,
    jwt,
    user,
  }));

  store.on(signOut, (state) => ({
    ...state,
    jwt: null,
    user: null,
  }));

  store.watch(({ jwt, user }) => {
    localforage.setItem('jwt', jwt);
    localforage.setItem('user', user);
  });

  return store;
}

export default createAuth();
