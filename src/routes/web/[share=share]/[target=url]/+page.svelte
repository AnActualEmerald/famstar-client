<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import settings from "$lib/store/settings";
  import { onMount } from "svelte";


  onMount(async () => {
    $settings.share = $page.data.share;
    $settings.target = $page.data.target;
    await goto("/", { replaceState: false });
  })

</script>