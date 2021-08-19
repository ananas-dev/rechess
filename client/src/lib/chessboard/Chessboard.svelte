<script>
  import { onMount } from "svelte";
  import * as d3 from "d3";
  import { Chess } from "chess.js";

  const chess = new Chess();
  const board = chess.board();
</script>

<div class="chessboard">
  {#each board as row, file}
    {#each row as square, rank}
      <div class={`square ${((file + 1 * 8 + rank) % 2) == 0 ? "w" : "b"}`}>
        {#if square != null}
          {square.type}
        {/if}
      </div>
    {/each}
  {/each}
</div>

<style>
  .chessboard {
    width: 640px;
    height: 640px;
  }
  .square {
    float: left;
    width: 80px;
    height: 80px;
    font-size: 50px;
    text-align: center;
    display: table-cell;
    vertical-align: middle;
  }

  .b {
    background-color: #b58863;
  }
  .w {
    background-color: #f0d9b5;
  }
</style>
