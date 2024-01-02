<script lang="ts">
  import { getContext, setContext } from 'svelte';
  import { writable } from 'svelte/store';

  let ccounter = writable<number>(0);
  setContext('counter', ccounter);

  export function getCounter() {
    return getContext<number>('counter');
  }
</script>

<slot />
