<script lang="ts">
  import "../app.css";
  // Potentially import a Navbar and Footer component here later
</script>

<div class="min-h-screen bg-gray-100">
  <!-- <Navbar /> -->
  <main class="container mx-auto py-8 px-4">
    <slot />
  </main>
  <!-- <Footer /> -->
</div>

<style lang="postcss">
  /* You can add global styles here if needed, or in app.css */
</style>
