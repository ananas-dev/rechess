<script lang="ts">
  import Button from "../components/Button.svelte";
  import { Chess } from "chess.js";
  import ChessBoard from "../components/ChessBoard.svelte";
  import { onMount, onDestroy } from "svelte";
  import websocket from "../stores/websocket";

  import type { MoveEvent } from "../types/ChessBoard";
  import type { Square } from "chess.js";
  import type { Unsubscriber } from "svelte/store";

  let messages: string[] = [];

  let unsub: Unsubscriber;

  onMount(() => {
    websocket.create()
    unsub = websocket.subscribe((currentMessage) => {
      messages = [...messages, currentMessage];
    });
  });

  const handleMove = (e: CustomEvent<MoveEvent>) => {
    const { from, to, cg, chess } = e.detail;
    // Move the chessboard
    chess.move({ from: from as Square, to: to as Square});
    websocket.sendMessage(
      JSON.stringify({
        type: "move",
        san: `${from}${to}`,
        fen: chess.fen(),
      })
    );
  };

  onDestroy(() => {
    unsub
    websocket.destroy()
  });
</script>

<div
  class="flex flex-col justify-center items-center text-center p-4 max-w-xs mx-auto sm:max-w-none"
>
  <div class="my-auto">
    <ChessBoard width="500px" height="500px" on:move={handleMove} />
  </div>
  {messages}
</div>
