<script lang="ts">
  export let type: "primary" | "secondary" = "primary";
  export let flat = false;
</script>

<button
  class={`
    transition duration-300 ease-in-out
    bg-${type}
    py-2 px-7
    bg-green-500
    text-white
    font-semibold
    rounded-lg
    hover:bg-green-600
    transform active:scale-95
  `}
  class:shadow-md={!flat}
>
  <slot />
</button>
