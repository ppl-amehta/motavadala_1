<script lang="ts">
  import "../app.css";
  import Navbar from "$lib/components/Navbar.svelte";
  import Footer from "$lib/components/Footer.svelte";
</script>

<div class="min-h-screen flex flex-col bg-gray-100">
  <Navbar />
  <main class="flex-grow container mx-auto py-8 px-4">
    <slot />
  </main>
  <Footer />
</div>

<style lang="postcss">
  /* You can add global styles here if needed, or in app.css */
</style>
