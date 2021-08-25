<script lang="ts">
  import Button from "../components/Button.svelte";
  import NavBar from "../components/NavBar.svelte";
  import { onMount, onDestroy } from "svelte";
  import websocket from "../stores/websocket";
  import { push } from "svelte-spa-router";
  import type { Unsubscriber } from "svelte/store";
  import { dataset_dev } from "svelte/internal";

  let messages: string[] = [];

  let unsub: Unsubscriber;

  let rooms = [];
  let gradient: string;

  onMount(() => {
    websocket.create("/");
    unsub = websocket.subscribe((currentMessage) => {
      messages = [...messages, currentMessage];

      try {
        const msg = JSON.parse(currentMessage);
        console.log(msg);

        switch (msg.type) {
          case "list":
            rooms = msg.rooms;
            break;
          case "create":
            push(`/play/${msg.room_id}`);
            break;
          default:
            break;
        }
      } catch {}
    });
  });

  onDestroy(() => {
    websocket.destroy();
    unsub();
  });

  const handleCreateGame = () => {
    websocket.sendMessage(
      JSON.stringify({
        type: "create",
      })
    );
  };
</script>

<div class="text-center p-4 max-w-xs mx-auto sm:max-w-none">
  <div class="m-10">
    <h1 class="text-6xl">Rechess</h1>
    <p class="text-xl text-gray-600">A cozy chess website</p>
  </div>
  <div on:click={handleCreateGame}>
    <Button>Create a game</Button>
  </div>
  <div class="container mx-auto m-10 grid grid-flow-row gap-4 lg:grid-cols-6 lg:grid-rows-2 md:grid-cols-2 md:grid-rows-6 sm:grid-cols-1 sm:grid-rows-12">
      {#each rooms as room}
        <div>
          <a href={`#/play/${room}`}>
            <div
              class="overflow-hidden rounded-lg shadow-md hover:shadow-xl transition duration-301 ease-in-out bg-green-500 hover:bg-green-600"
            >
              <header
                class="flex items-center justify-center leading-tight p-2 md:p-4"
              >
                <h1 class="text-lg text-white">
                  {room}
                </h1>
              </header>
            </div>
          </a>
        </div>
      {/each}
  </div>
</div>
