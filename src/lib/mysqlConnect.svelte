<script lang="ts">
  import Button, { Icon, Label } from '@smui/button';
  import Textfield from '@smui/textfield';
  import { invoke } from '@tauri-apps/api';

  let dsn = '';
  let dsnResult = '';

  async function mysql_connect() {
    try {
      dsnResult = await invoke('invoke_handler', { func: 'mysql_connect', data: dsn });
    } catch (e) {
      if (e instanceof Error) {
        dsnResult = 'ERROR: ' + e.message;
      } else {
        dsnResult = 'Unknown error occured' + e;
      }
    }
  }
</script>

<div>
  <Textfield
    type="text"
    bind:value={dsn}
    label="Enter your name"
    style="width: 100%;"
    helperLine$style="width: 100%;"
    variant="outlined"
  ></Textfield>
  <br />
  <Button color="primary" variant="unelevated" on:click={mysql_connect}>
    <Icon class="material-icons">favorite</Icon>
    <Label>Connect To Mysql</Label>
  </Button>
  <p>Result: {dsnResult}</p>
</div>
