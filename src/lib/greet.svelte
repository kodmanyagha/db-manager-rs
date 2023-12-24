<script lang="ts">
  import Button, { Label, Icon } from '@smui/button';
  import { invoke } from '@tauri-apps/api';
  import Textfield from '@smui/textfield';
  import HelperText from '@smui/textfield/helper-text';

  let name = '';
  let greetMsg = '';
  let focused = false;

  async function greet() {
    greetMsg = await invoke('greet', { name });
  }
</script>

<div>
  <Textfield
    type="text"
    bind:value={name}
    label="Enter your name"
    style="width: 100%;"
    helperLine$style="width: 100%;"
    variant="outlined"
    on:focus={() => (focused = true)}
    on:blur={() => (focused = false)}
  ></Textfield>
  <br />
  <Button color="primary" variant="unelevated" on:click={greet}>
    <Icon class="material-icons">favorite</Icon>
    <Label>Greet</Label>
  </Button>
  <p>{greetMsg}</p>
</div>
