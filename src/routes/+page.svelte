<script context="module" lang="ts">
  import { images } from "$lib/store/ImageStore";
  import { messages } from "$lib/store/MessageStore";
</script>

<script lang="ts">
  import ImageItem from "$lib/components/Image.svelte";
  import MessageItem from "$lib/components/Message.svelte";
  import Carousel from "svelte-carousel";
  import { browser } from "$app/environment";
  import { init } from "$lib/sync";
  import { onMount } from "svelte";

  onMount(async () => {
    if (browser) {
      const { invoke } = await import("@tauri-apps/api");
      const target: string = await invoke("get_target");
      const share: string = await invoke("get_share");
      init(share, target);
    }
  });
</script>

<main>
  {#if browser}
    {#key $images}
      {#key $messages}
        <Carousel
          autoplay
          dots={false}
          autoplayDuration={30000}
          duration={5000}
          arrows={false}
          pauseOnFocus
        >
          {#each $images as data}
            <ImageItem data={data.content} />
          {/each}
          {#each $messages as msg}
            <MessageItem msg={msg.content} />
          {/each}
        </Carousel>
      {/key}
    {/key}
  {/if}
</main>

<style>
  main {
    text-align: center;
    max-width: 240px;
    margin: 0 auto;
  }

  /* h1 {
        color: #ff3e00;
        text-transform: uppercase;
        font-size: 4em;
        font-weight: 100;
    } */

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
