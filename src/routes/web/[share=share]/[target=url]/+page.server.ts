// <script context="module" lang="ts">
import { init } from "$lib/sync";
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from "./$types";

export const prerender = false;

export const load: PageLoad = ({params}) => {
    return {
      share: params.share,
      target: params.target,
      ready: true
    }
};
// </script>