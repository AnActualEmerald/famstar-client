<script context="module" lang="ts">
  import { images } from "./store/ImageStore";
  import { messages } from "./store/MessageStore";
</script>

<script lang="ts">
  import ImageItem from "./components/Image.svelte";
  import MessageItem from "./components/Message.svelte";
  import Carousel from "svelte-carousel";
  import { onMount } from "svelte";
  // import Message from "./components/Message.svelte";

  // let imagesMax = 0;
  // let messagesMax = 1;
  // $: {
  //   const ratio = $images.length / $messages.length;
  //   imagesMax = Math.floor(ratio);
  // }
  let carousel;

  onMount(() => {
    const socket = new WebSocket("ws://localhost:9000");
    socket.addEventListener("open", (e) => {
      console.log(`Opened websocket connection`);
      //sync for 10 minutes, dont for 50 minutes
      let start = () => {
        socket.send("start");
        setTimeout(() => {
          stop();
        }, 600000);
      };
      let stop = () => {
        socket.send("stop");
        setTimeout(() => {
          start();
        }, 3000000);
      };
      start();
    });
    socket.addEventListener("error", (_) => console.log(`Websocket error`));

    socket.addEventListener("message", (e) => {
      console.log("Recieved message");
      //handle receiving in messages and images
      try {
        const j = JSON.parse(e.data);

        messages.update((curr): string[] => {
          return [j.content as string, ...curr];
        });
      } catch {
        images.update((curr): Blob[] => {
          return [e.data as Blob, ...curr];
        });
      }
    });
  });
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
        bind:this={carousel}
      >
        {#each $images as data}
          <ImageItem {data} />
        {/each}
        {#each $messages as msg}
          <MessageItem {msg} />
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
