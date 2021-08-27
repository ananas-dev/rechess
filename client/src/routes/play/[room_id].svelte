<script context="module">
  import { devServerHost, devServerPort } from "$lib/util/env";
  export const load = async ({ page }) => {
    let fen = await fetch(
      `http://${devServerHost}:${devServerPort}/api/v1/rooms/${page.params.room_id}`
    );

    return {
      props: {
        room_id: page.params.room_id,
        room_info: await fen.json(),
      },
    };
  };
</script>

<script lang="ts">
  import ChessBoard from "$lib/components/ChessBoard.svelte";
  import Button from "$lib/components/Button.svelte";
  import { wsBuilder } from "$lib/util/websocket";
  import { onMount, onDestroy } from "svelte";

  import type { MoveEvent } from "$lib/types/ChessBoard";
  import type { Square } from "chess.js";
  import type { Unsubscriber } from "svelte/store";
  import type { Color } from "chessground/types";

  export let room_id: string;
  export let room_info: any;

  let unsub: Unsubscriber;

  let inGame = false;
  let orientation: Color;
  let movableSide: Color;
  let turnColor: Color;
  let fen: string = room_info.fen;

  let moveCommand;
  let loadCommand;

  let socket: WebSocket;

  onMount(async () => {
    if (room_info.fen) {
      inGame = true;
    }

    socket = wsBuilder(`/play/${room_id}`);
    console.log(room_info);
    socket.onmessage = ({ data }) => {
      try {
        const msg = JSON.parse(data);
        console.log(msg);

        switch (msg.type) {
          case "move":
            moveCommand(msg.from, msg.to);
            break;
          case "start":
            orientation = msg.color;
            movableSide = msg.color;
            turnColor = "white";
            inGame = true;
            break;
          case "reconnect":
            inGame = true;
            orientation = msg.color;
            movableSide = msg.turn;
            turnColor = msg.turn;
            fen = msg.fen;
            loadCommand(msg.fen);
            break;
        }
      } catch (e) {
        console.error(e);
      }
    };
  });

  const handleMove = (e: CustomEvent<MoveEvent>) => {
    const { from, to, cg, chess } = e.detail;
    // Move the chessboard
    chess.move({ from: from as Square, to: to as Square });

    socket.send(
      JSON.stringify({
        type: "move",
        from,
        to,
        fen: chess.fen(),
      })
    );
  };

  const copyStringToClipboard = (str: string) => {
    // Create new element
    var el = document.createElement("textarea");
    // Set value (string to be copied)
    el.value = str;
    // Set non-editable to avoid focus and move outside of view
    el.setAttribute("readonly", "");
    document.body.appendChild(el);
    // Select text inside element
    el.select();
    // Copy text to clipboard
    document.execCommand("copy");
    // Remove temporary element
    document.body.removeChild(el);
  };

  onDestroy(() => {
    if (socket) {
      socket.close();
    }
  });
</script>

<div
  class="flex flex-col justify-center items-center text-center p-4 max-w-xs mx-auto my-auto sm:max-w-none"
>
  {#if inGame}
    <div>
      <ChessBoard
        width="80vh"
        height="80vh"
        {orientation}
        {movableSide}
        {turnColor}
        {fen}
        on:move={handleMove}
        bind:load={loadCommand}
        bind:move={moveCommand}
      />
    </div>
  {:else}
    <div
      on:click={() => {
        copyStringToClipboard(document.location.href);
      }}
    >
      <Button>Copy invite</Button>
    </div>
  {/if}
</div>
