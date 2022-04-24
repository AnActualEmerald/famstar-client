<script lang="ts">
  export let msg: string;
</script>

<div class="carouselItem">
  <p>{msg}</p>
</div>

<style>
  .carouselItem {
    text-align: center;
    display: flex;
    /* flex-direction: column; */
    justify-content: space-evenly;
    height: 100%;
  }
  p {
    margin-top: 240px;
  }
</style>
