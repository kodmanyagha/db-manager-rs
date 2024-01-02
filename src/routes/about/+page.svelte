<script lang="ts">
  import { invoke } from '@tauri-apps/api';

  let result = '';

  async function example_json() {
    try {
      result = await invoke('invoke_handler', {
        func: 'example_json',
        data: JSON.stringify({
          u1: 10,
          i1: 20,
          s1: '30',
        }),
      });
    } catch (e) {
      if (e instanceof Error) {
        result = 'ERROR: ' + e.message;
      } else {
        result = 'Unknown error occured' + e;
      }
    }
  }
</script>

<svelte:head>
  <title>About</title>
  <meta name="description" content="About this app" />
</svelte:head>

<div class="text-column">
  <h1>About this app</h1>

  <div>
    <button class="btn btn-primary" on:click={example_json}> Click Me </button>
    <p>Result: {result}</p>
  </div>

  <p>
    This is a <a href="https://kit.svelte.dev">SvelteKit</a> app. You can make your own by typing the
    following into your command line and following the prompts:
  </p>

  <pre>npm create svelte@latest</pre>

  <p>
    The page you're looking at is purely static HTML, with no client-side interactivity needed.
    Because of that, we don't need to load any JavaScript. Try viewing the page's source, or opening
    the devtools network panel and reloading.
  </p>
</div>
