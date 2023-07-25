import { writable } from "svelte/store";

export type Message = {
  path: string;
  content: string;
};
export const messages = writable<Message[]>([]);
