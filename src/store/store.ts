import { writable } from 'svelte/store';

export const counter = writable<number>(0);

export type User = {
  id: bigint;
  firstname: string;
  lastname: string;
  username: string;
  mail: string;
  gender: 'male' | 'female' | 'not_specified';
};

export const user = writable<User | null>(null);
