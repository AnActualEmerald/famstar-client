<script lang="ts">
  export let msg: string;
</script>

<div class="carouselItem">
  <p>{msg}</p>
</div>

<style>
  .carouselItem {
    text-align: center;
    height: 100vh;
  }

  p {
    position: relative;
    top: 50vh;
    transform: translateY(-50%);
    color: white;

    user-select: none;

    /* background-color: darkslategray; */
    /* border-radius: 10px; */
    /* padding: 20px; */
  }
</style>
