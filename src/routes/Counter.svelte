<script lang="ts">
  import { Button } from '@sveltestrap/sveltestrap';
  import { spring } from 'svelte/motion';
  import { counter } from '../store/store';

  const displayed_count = spring();

  /* We can subscribe for changes in state, but better way is using dollar syntax.
  We can auto subscribe with this way. */
  //let count = 0;
  //counter.subscribe((val) => (count = val));
  //$: displayed_count.set(count);

  // Use dollar syntax for subscribing to counter state.
  $: displayed_count.set($counter);
  $: offset = modulo($displayed_count, 1);

  function modulo(n: number, m: number) {
    // handle negative numbers
    return ((n % m) + m) % m;
  }
</script>

<div class="counter">
  <Button
    type="button"
    color="primary"
    on:click={() => counter.update((prev) => prev - 1)}
    aria-label="Decrease the counter by one"
  >
    <i class="fa-solid fa-minus"></i>
  </Button>

  <div class="counter-viewport">
    <div class="counter-digits" style="transform: translate(0, {100 * offset}%)">
      <strong class="hidden" aria-hidden="true">{Math.floor($displayed_count + 1)}</strong>
      <strong>{Math.floor($displayed_count)}</strong>
    </div>
  </div>

  <Button
    type="button"
    color="primary"
    on:click={() => counter.update((prev) => prev + 1)}
    aria-label="Increase the counter by one"
  >
    <i class="fa-solid fa-plus"></i>
  </Button>
</div>
<Button on:click={() => counter.set(0)} type="button" color="success" outline class="w-100 btn-lg">
  Reset
</Button>

<style>
  .counter {
    display: flex;
    border-top: 1px solid rgba(0, 0, 0, 0.1);
    border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    margin: 1rem 0;
  }

  .counter-viewport {
    width: 8em;
    height: 4em;
    overflow: hidden;
    text-align: center;
    position: relative;
  }

  .counter-viewport strong {
    position: absolute;
    display: flex;
    width: 100%;
    height: 100%;
    font-weight: 400;
    color: var(--color-theme-1);
    font-size: 4rem;
    align-items: center;
    justify-content: center;
  }

  .counter-digits {
    position: absolute;
    width: 100%;
    height: 100%;
  }

  .hidden {
    top: -100%;
    user-select: none;
  }
</style>
