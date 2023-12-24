<script>
  import Footer from '../lib/components/footer/footer.svelte';
  import Header from '../lib/components/header/header.svelte';
  import './styles.css';
</script>

<div class="container-fluid">
  <Header />

  <main>
    <slot />
  </main>

  <Footer />
</div>
