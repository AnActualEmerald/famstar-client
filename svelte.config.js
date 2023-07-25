import adapter from '@sveltejs/adapter-static'
import preprocess from "svelte-preprocess"

/** @type {import('@sveltejs/kit').Config} */
export default {
    kit: {
        adapter: adapter(),
        prerender: {
            handleHttpError: "warn"
        }   
    },
    preprocess: preprocess()
}