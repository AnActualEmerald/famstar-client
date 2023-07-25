<script lang="ts">
  export let data: Blob;

  let src = URL.createObjectURL(data);
</script>

<div class="carouselItem">
  <img {src} alt="Lovely family" />
</div>
