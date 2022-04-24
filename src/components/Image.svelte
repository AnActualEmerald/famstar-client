<script lang="ts">
    let data: Blob;
    export let url: string;

    let src;
    if (data) {
        src = URL.createObjectURL(data);
    } else if (url) {
        src = url;
    }
</script>

<div class="carouselItem">
    <img {src} alt="Lovely family" />
</div>
