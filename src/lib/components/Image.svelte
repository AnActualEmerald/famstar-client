<script lang="ts">
  export let data: Blob;

  let src = URL.createObjectURL(data);
</script>

<div class="carouselItem">
  <img {src} alt="Lovely family" />
</div>

<style>
  .carouselItem {
    min-height: 100vh;
  }
  img {
    max-width: 480px;
    max-height: 800px;
    width: auto;
    height: auto;

    position: relative;
    top: 50vh;
    transform: translateY(-50%);
  }
</style>
