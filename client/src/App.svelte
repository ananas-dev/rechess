<script lang="ts">
  //import Router from "./components/router/Router.svelte";
  //import Route from "./components/router/Route.svelte";

  import Router, {wrap} from "svelte-spa-router";

  import Home from "./routes/Home.svelte";
  import Play from "./routes/Play.svelte";
  import NavBar from "./components/NavBar.svelte";


const routes = {
    // Exact path
    '/': Home,

    // Using named parameters, with last being optional
    '/play/:room_id':  Play,
}


</script>

<main>
  <Router {routes} />
</main>

<style lang="postcss" global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
