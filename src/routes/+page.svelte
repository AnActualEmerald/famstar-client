<script context="module" lang="ts">
  import { images, type Image } from "$lib/store/ImageStore";
  import { messages, type Message } from "$lib/store/MessageStore";
</script>

<script lang="ts">
  import ImageItem from "$lib/components/Image.svelte";
  import MessageItem from "$lib/components/Message.svelte";
  import Carousel from "svelte-carousel";
  import { browser } from "$app/environment";
  import { init, localReplica } from "$lib/sync";
  import { onMount } from "svelte";
  import { ReplicaCache, notErr } from "@forge/earthstar";
  import { invoke } from "@tauri-apps/api";
  import { appWindow } from "@tauri-apps/api/window";

  let replicaCache: ReplicaCache | null = null;

  const updateStores = async () => {
    const messageDocs = replicaCache
      .queryDocs({
        historyMode: "latest",
        filter: {
          pathStartsWith: "/messages",
        },
      })
      .filter((v) => v.text.length > 0);

    const imageDocs = replicaCache
      .queryDocs({
        historyMode: "latest",
        filter: {
          pathStartsWith: "/images",
        },
      })
      .filter((v) => v.text.length > 0);

    messages.update((prev) => {
      console.log(prev);
      return messageDocs.map((v) => {
        return {
          path: v.path,
          content: v.text,
        } as Message;
      });
    });

    let tmp: Image[] = [];
    for (const doc of imageDocs) {
      const img = replicaCache.getAttachment(doc);
      if (notErr(img)) {
        const bytes = await img.bytes();
        tmp.push({
          path: doc.path,
          content: new Blob([bytes]),
        });
      }
    }
    $images = tmp;
  };

  onMount(async () => {
    if (import.meta.env.PROD) {
      await appWindow.setAlwaysOnTop(true);
      await appWindow.setFullscreen(true);
      await appWindow.setCursorGrab(true);
      await appWindow.setCursorVisible(false);
    }

    const target: string = await invoke("get_target");
    const share: string = await invoke("get_share");
    init(share, target);

    replicaCache = new ReplicaCache(localReplica);
    replicaCache.onCacheUpdated(async () => {
      await updateStores();
    });

    await updateStores();
  });
</script>

<main>
  {#key $images}
    {#key $messages}
      <Carousel
        autoplay
        dots={false}
        autoplayDuration={30000}
        duration={3000}
        arrows={false}
        pauseOnFocus
      >
        {#each $images as img}
          <ImageItem data={img.content} />
        {/each}
        {#each $messages as msg}
          <MessageItem msg={msg.content} />
        {/each}
      </Carousel>
    {/key}
  {/key}
</main>

<style>
  :global(:root) {
    background-color: black;
    color: white;
    padding: 0;
    margin: 0;
  }

  :global(body) {
    height: 100%;
    margin: 0;
    padding: 0;
  }

  main {
    text-align: center;
    min-height: 100%;
  }
</style>
