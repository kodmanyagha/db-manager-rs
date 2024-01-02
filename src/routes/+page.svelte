<script lang="ts">
  import MysqlConnect from '$lib/mysql-connect.svelte';
  import { Button } from '@sveltestrap/sveltestrap';
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
        <p>Hello World, how's going?</p>

        <Button type="button" color="primary" class="w-100 btn-lg">
          <i class="fa-solid fa-house"></i>
          <br />
          This is sveltestrap button!!!
          <br />
          This is fine.
        </Button>
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
        <Counter />
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

          <button class="btn btn-primary"> This button has a Class </button>
        </p>
        <a href="mui-example/treeview" class="w-100 btn btn-lg btn-primary">Tree View</a>
      </div>
    </div>
  </div>
</div>
