<script lang="ts">
  import Button from "../components/Button.svelte";
  import { onMount, onDestroy } from "svelte";
  import websocket from "../stores/websocket";
  import type { Unsubscriber } from "svelte/store";

  let messages: string[] = [];

  let unsub: Unsubscriber;

  onMount(() => {
    unsub = websocket.subscribe((currentMessage) => {
      messages = [...messages, currentMessage];
    });
  });

  onDestroy(unsub);
</script>

<div class="text-center p-4 max-w-xs mx-auto sm:max-w-none">
  <Button>
    Click Me
  </Button>
</div>
