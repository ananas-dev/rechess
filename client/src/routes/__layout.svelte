<script>
  import Header from "$lib/header/Header.svelte";
  import "../app.css";
</script>

<Header />

<main class="main-wrap">
  <slot />
</main>

<style>
  .main-wrap {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 1rem;
    width: 100%;
    height: 100%;
    margin: 0 auto;
    box-sizing: border-box;
  }
</style>
