<script lang="ts">
    import { goto } from "$app/navigation";
    import settings from "$lib/store/settings";
    import { invoke } from "@tauri-apps/api";
    import { appWindow } from "@tauri-apps/api/window";
    import { onMount } from "svelte";
    
    onMount(async () => {
        if (import.meta.env.PROD){
            await appWindow.setFullscreen(true);
            await appWindow.setAlwaysOnTop(true);
            await appWindow.setCursorGrab(true);
            await appWindow.setCursorVisible(true);
            await appWindow.setFocus();
        }
        $settings.share = await invoke('get-share');
        $settings.target = await invoke('get-target');
        $settings.ready = true;
        await goto('/');
    });

</script>