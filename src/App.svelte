<script context="module" lang="ts">
    import { images } from "./store/ImageStore";
    import ImageItem from "./components/Image.svelte";
    import MessageItem from "./components/Message.svelte";
    //connect to syncer
    const socket = new WebSocket("ws://localhost:9000");
    socket.addEventListener("open", (e) => {
        console.log(`Opened websocket connection ${e}`);
        socket.send(
            JSON.stringify({
                message: "start",
            })
        );
    });
    socket.addEventListener("error", (e) =>
        console.log(`Websocket error: ${e}`)
    );

    socket.addEventListener("message", (e) => {
        //handle receiving in messages and images
        console.log(e.data);
    });
</script>

<script lang="ts">
    import Carousel from "svelte-carousel";
</script>

<main>
    <Carousel
        autoplay
        dots={false}
        autoplayDuration={5000}
        duration={1000}
        arrows={false}
        autoplayProgressVisible
        timingFunction="ease-in"
        pauseOnFocus
    >
        {#each $images as url}
            <!-- <ImageItem {url} /> -->
        {/each}
    </Carousel>
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
