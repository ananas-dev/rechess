<script context="module">
  import { devServerHost, devServerPort } from "$lib/util/env";
  export const load = async ({ page }) => {
    let fen = await fetch(
      `http://localhost:5000/api/v1/rooms/${page.params.room_id}`
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
  import { toDests } from "$lib/util/chess";
  import { onMount, onDestroy } from "svelte";
  import { page } from "$app/stores";

  import type { MoveEvent } from "$lib/types/ChessBoard";
  import type { Color } from "chessground/types";

  export let room_id: string;
  export let room_info: any;

  let orientation: Color;
  let movableSide: Color;
  let turnColor: Color;
  let fen: string = room_info.fen;
  let dests;
  let endStatus;

  let socket: WebSocket;

  enum GameState {
    NotStarted,
    Started,
    Ended,
  }

  let state: GameState;

  onMount(async () => {
    state = room_info.fen ? GameState.Started : GameState.NotStarted;

    socket = wsBuilder(`/play/${room_id}`);
    console.log(room_info);
    socket.onmessage = ({ data }) => {
      try {
        const msg = JSON.parse(data);
        console.log(msg);

        switch (msg.type) {
          case "move":
            turnColor = msg.side;
            if (msg.dests) {
              dests = toDests(msg.dests);
            }
            fen = msg.fen;
            break;
          case "start":
            state = GameState.Started;
            orientation = msg.color;
            movableSide = msg.color;
            turnColor = "white";
            if (msg.dests) {
              dests = toDests(msg.dests);
            }
            break;
          case "reconnect":
            state = GameState.Started;
            orientation = msg.color;
            movableSide = msg.color;
            turnColor = msg.turn;

            if (msg.dests) {
              dests = toDests(msg.dests);
            }

            fen = msg.fen;
            break;
          case "game_end":
            state = GameState.Ended;
            endStatus = msg.result;
        }
      } catch (e) {
        console.error(e);
      }
    };
  });

  const handleMove = (e: CustomEvent<MoveEvent>) => {
    const { from, to, cg } = e.detail;
    socket.send(
      JSON.stringify({
        type: "move",
        from,
        to,
        fen: cg.getFen(),
      })
    );
  };

  // TODO: Move to some util file
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

<svelte:head>
  <meta title="og:title" content="Join a chess game!"/>
  <meta title="og:website" content="rechess.org"/>
  <meta title="og:type" content="website"/>
  <meta title="og:url" content={$page.host + $page.path}/>
</svelte:head>

<div
  class="flex flex-col justify-center items-center text-center p-4 max-w-xs mx-auto my-auto sm:max-w-none"
>
  {#if state == GameState.Started || state == GameState.Ended}
    <div>
      <ChessBoard
        width="80vh"
        height="80vh"
        {orientation}
        {movableSide}
        {turnColor}
        {fen}
        {dests}
        on:move={handleMove}
      />
      {#if state == GameState.Ended}
        <div>
          Game over: {endStatus}
          <Button>
            <a sveltekit:prefetch href="/">
              Go back to the lobby
            </a>
          </Button>
        </div>
      {/if}
    </div>
  {:else if state == GameState.NotStarted}
    <div
      on:click={() => {
        copyStringToClipboard(document.location.href);
      }}
    >
      <Button>Copy invite</Button>
    </div>
  {/if}
</div>