<script>
  import Greet from '$lib/greet.svelte';
  import MysqlConnect from '$lib/mysqlConnect.svelte';
  import Button, { Label } from '@smui/button';
  import Counter from './Counter.svelte';
</script>

<svelte:head>
  <title>Home</title>
  <meta name="description" content="Home page" />
</svelte:head>

<div class="row row-cols-1 row-cols-md-3 mb-3 text-center">
  <div class="col">
    <div class="card mb-4 rounded-3 shadow-sm">
      <div class="card-header py-3">
        <h4 class="my-0 fw-normal">Free</h4>
      </div>
      <div class="card-body">
        <h1 class="card-title pricing-card-title">
          $0<small class="text-body-secondary fw-light">/mo</small>
        </h1>
        <p>
          <Greet />
        </p>
        <button type="button" class="w-100 btn btn-lg btn-outline-primary">Greetings</button>
      </div>
    </div>
  </div>

  <div class="col">
    <div class="card mb-4 rounded-3 shadow-sm">
      <div class="card-header py-3">
        <h4 class="my-0 fw-normal">Free</h4>
      </div>
      <div class="card-body">
        <h1 class="card-title pricing-card-title">
          $0<small class="text-body-secondary fw-light">/mo</small>
        </h1>
        <p>
          <Counter />
        </p>
        <button type="button" class="w-100 btn btn-lg btn-outline-primary">Counter</button>
      </div>
    </div>
  </div>

  <div class="col">
    <div class="card mb-4 rounded-3 shadow-sm">
      <div class="card-header py-3">
        <h4 class="my-0 fw-normal">Free</h4>
      </div>
      <div class="card-body">
        <h1 class="card-title pricing-card-title">
          $0<small class="text-body-secondary fw-light">/mo</small>
        </h1>
        <p>
          <MysqlConnect />

          <Button class="myClass"><Label>This button has a Class</Label></Button>
        </p>
        <a href="mui-example/accordion" class="w-100 btn btn-lg btn-outline-primary">Counter</a>
      </div>
    </div>
  </div>
</div>
