<script context="module">
  export const load = ({ page }) => {
    return {
      props: {
        room_id: page.params.room_id,
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

  let unsub: Unsubscriber;

  let started = false;
  let orientation: Color;
  let movableSide: Color;
  let turnColor: Color;
  let room_url;

  let move_command;

  let socket: WebSocket;

  onMount(() => {
    socket = wsBuilder(`/play/${room_id}`)
    socket.onmessage = ({data}) => {
      try {
        const msg = JSON.parse(data);
        console.log(msg);

        switch(msg.type) {
          case "start":
            orientation = msg.color;
            movableSide = msg.color;
            turnColor = "white";
            started = true;
            break;
          case "move":
            move_command(msg.from, msg.to);
            break;
        }
      } catch(e) {
        console.error("Received invalid json data!");
      }
    }
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
  })
</script>

<div
  class="flex flex-col justify-center items-center text-center p-4 max-w-xs mx-auto my-auto sm:max-w-none"
>
  {#if started}
    <div>
      <ChessBoard width="80vh" height="80vh" {orientation} {movableSide} {turnColor} on:move={handleMove} bind:move={move_command} />
    </div>
  {:else}
    <div on:click={() => { copyStringToClipboard(document.location.href)}}>
      <Button>Copy invite</Button>
    </div>
  {/if}
</div>
