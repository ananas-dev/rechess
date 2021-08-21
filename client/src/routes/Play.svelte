<script lang="ts">
  import Button from "../components/Button.svelte";
  import { Chess } from "chess.js";
  import ChessBoard from "../components/ChessBoard.svelte";
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

  const handleMove = (e: any) => {
    const { from, to, cg, chess } = e.detail;
    // Move the chessboard
    chess.move({ from, to });
    websocket.send_message(JSON.stringify({
      type: "move",
      san: `${from}${to}`,
      fen: chess.fen(),
    }));
  };

  onDestroy(unsub);
</script>

<div
  class="flex flex-col justify-center items-center text-center p-4 max-w-xs mx-auto sm:max-w-none"
>
  <div class="m-5">
    <ChessBoard width="500px" height="500px" on:move={handleMove} />
  </div>
  <div class="flex-initial">
    <Button>Click Me</Button>
  </div>
  {messages}
</div>
