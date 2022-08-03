<script context="module" lang="ts">
  import { images, Image } from "./store/ImageStore";
  import { messages, Message } from "./store/MessageStore";
</script>

<script lang="ts">
  import ImageItem from "./components/Image.svelte";
  import MessageItem from "./components/Message.svelte";
  import Carousel from "svelte-carousel";
  import { onMount } from "svelte";
  import { listen, emit } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";

  const log = async (msg) => {
    await invoke("log", { msg });
  };
  // import Message from "./components/Message.svelte";

  // let imagesMax = 0;
  // let messagesMax = 1;
  // $: {
  //   const ratio = $images.length / $messages.length;
  //   imagesMax = Math.floor(ratio);
  // }
  //let carousel;
  function toUint8Array(input: string): Uint8Array {
    return new Uint8Array(
      input.split("").map(function (c) {
        return c.charCodeAt(0);
      })
    );
  }

  onMount(async () => {
    await log("App mounted");
    const unlisten = await listen("sync-event", (e: any) => {
      log("Got sync event");
      const doc = e.payload;
      log(`Handling sync event type ${doc.type}`);
      if (doc.type) {
        if (doc.type == "addImage") {
          images.update((curr): Image[] => {
            let raw = toUint8Array(atob(doc.content as string));
            return [
              { path: doc.path as string, content: new Blob([raw]) },
              ...curr,
            ];
          });
        } else if (doc.type == "addMessage") {
          messages.update((curr): Message[] => {
            return [
              { path: doc.path as string, content: doc.content as string },
              ...curr,
            ];
          });
        } else if (doc.type == "removeDoc") {
          if (doc.path.startsWith("image")) {
            images.update((curr): Image[] => {
              return curr.filter((e) => e.path !== doc.path);
            });
          } else {
            messages.update((curr): Message[] => {
              return curr.filter((e) => e.path !== doc.path);
            });
          }
        }
      }
    });
    log("Ready, starting");
    await invoke("start");
  });
  const key = {
    img: $images,
    msg: $messages,
  };
</script>

<main>
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
