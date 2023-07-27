import { writable } from "svelte/store";

export type Settings = {
    share?: string | undefined,
    target?: string | undefined,
    ready: boolean
};

export default writable({ready: false} as Settings);