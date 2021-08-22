<script lang="ts">
  import Button from "../components/Button.svelte";
  import NavBar from "../components/NavBar.svelte";
  import { onMount, onDestroy } from "svelte";
  import websocket from "../stores/websocket";
  import type { Unsubscriber } from "svelte/store";

  let messages: string[] = [];

  let unsub: Unsubscriber;

  onMount(() => {
    websocket.create();
    unsub = websocket.subscribe((currentMessage) => {
      messages = [...messages, currentMessage];
    });
  });

  onDestroy(() => {
    unsub;
    websocket.destroy();
  });
</script>

<div class="text-center p-4 max-w-xs mx-auto sm:max-w-none">
  <div class="m-10">
    <h1 class="text-6xl">Rechess</h1>
    <p class="text-xl text-gray-600">A cozy chess website</p>
  </div>
  <a href="#/play">
    <Button>Create a game</Button>
  </a>
</div>
