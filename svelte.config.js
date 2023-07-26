import adapter from '@sveltejs/adapter-static'
import preprocess from "svelte-preprocess"

/** @type {import('@sveltejs/kit').Config} */
export default {
    kit: {
        adapter: adapter({
            pages: 'build',
            assets: 'build',
            fallback: 'index.html',
            precompress: false,
            strict: true
        }), 
        paths: {
            base: ''
        }
    },
    preprocess: preprocess()
}