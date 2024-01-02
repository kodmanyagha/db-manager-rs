<script lang="ts">
  import { invokeTauri } from './utils';

  let dsn = '';
  let dsnResult = '';

  async function mysql_connect() {
    try {
      dsnResult = (await invokeTauri('mysql_connect', dsn)) as string;
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
  <p>Result: {dsnResult}</p>
</div>
