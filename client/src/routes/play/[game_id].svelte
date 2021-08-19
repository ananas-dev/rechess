<script context="module">
  export const load = ({ page, session }) => {
    return {
      props: {
        game_id: page.params.game_id,
      },
    };
  };
</script>

<script>
  import { onMount, onDestroy } from "svelte";
  import Chessboard from "$lib/chessboard/Chessboard.svelte";
  import websocket from "../../stores/websocket";

  export let game_id;

  let message;
  let messages = [];

  onMount(() => {
    websocket.connect(`ws://localhost:5000/ws/${game_id}"`);
  });

  const unsub = websocket.subscribe((currentMessage) => {
    messages = [...messages, currentMessage];
  });

  onDestroy(() => {
    unsub();
  });

</script>

<div>
  <Chessboard />
  {#each messages as msg}
    <p>
      {msg}
    </p>
  {/each}
</div>
