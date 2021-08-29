<script lang="ts">
  import Button from "$lib/components/Button.svelte";
  import { onMount, onDestroy } from "svelte";
  import { goto, prefetch } from "$app/navigation";
  import { wsBuilder } from "$lib/util/websocket";

  let rooms = [];

  let socket: WebSocket;

  onMount(async () => {
    socket = wsBuilder("/");
    socket.onmessage = ({ data }) => {
      try {
        const msg = JSON.parse(data);
        console.log(msg);

        switch (msg.type) {
          case "list":
            rooms = msg.rooms;
            break;
          case "create":
            goto(`/${msg.room_id}`);
            break;
          default:
            break;
        }
      } catch (e) {
        console.error("Received invalid json data!");
      }
    };
  });

  const handleCreateGame = () => {
    socket.send(
      JSON.stringify({
        type: "create",
      })
    );
  };

  onDestroy(() => {
    if (socket) {
      socket.close();
    }
  });
</script>

<div class="text-center p-4 max-w-xs mx-auto sm:max-w-none">
  <div class="m-10">
    <h1 class="text-6xl">Rechess</h1>
    <p class="text-xl text-gray-600">A comfy chess website ☕</p>
  </div>
  <div on:click={handleCreateGame}>
    <Button>Create a game</Button>
  </div>
  <div
    class="container mx-auto m-10 grid grid-flow-row gap-4 lg:grid-cols-6 lg:grid-rows-2 md:grid-cols-2 md:grid-rows-6 sm:grid-cols-1 sm:grid-rows-12"
  >
    {#each rooms as room}
      <div>
        <a sveltekit:prefetch href={`/${room}`}>
          <div
            class="overflow-hidden rounded-lg shadow-md hover:shadow-xl transition duration-300 ease-in-out bg-green-500 hover:bg-green-600"
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
