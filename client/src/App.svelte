<script lang="ts">
  import Home from "./routes/Home.svelte";
  import Play from "./routes/Play.svelte";
  import Router from "svelte-spa-router";
  import NavBar from "./components/NavBar.svelte";

  const routes = {
    "/": Home,
    "/play": Play,
    //'*': NotFound,
  };
</script>

<body>
  <NavBar />
  <Router {routes} />
</body>

<style lang="postcss" global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
