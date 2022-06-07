import { writable } from "svelte/store";

export type Image = {
  path: string;
  content: Blob;
};
export const images = writable<Image[]>([]);
