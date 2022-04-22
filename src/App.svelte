<script context="module">
  import { images } from "./store/ImageStore";
  //init image store
  //TODO: get image data from the core at startup
  //TODO: scale images for display on pi
  images.set(["/testdata/1.png", "/testdata/2.png"]);
</script>

<script>
  import Carousel from "svelte-carousel";
  //get images from core
  //Probably want to keep them in a Store
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
    {#each $images as src}
      <img {src} alt="Lovely family" />
    {/each}
  </Carousel>
</main>

<style>
  main {
    text-align: center;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }

  .Carousel {
    width: 300px;
  }
</style>
