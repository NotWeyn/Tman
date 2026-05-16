<script>
  import '../app.css';
</script>

<div class="app-container">
  <slot />
</div>

<style>
  .app-container {
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    background-color: var(--bg-main);
  }
</style>
