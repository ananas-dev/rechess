<script lang="ts">
  import { Router, Route } from "svelte-navigator";
  import Home from "./routes/Home.svelte";
  import Play from "./routes/Play.svelte";
</script>

<Router>
  <Route path="/">
    <Home />
  </Route>
  <Route path="play">
    <Play />
  </Route>
</Router>

<style lang="postcss" global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style>
