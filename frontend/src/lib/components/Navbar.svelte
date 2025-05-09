<script lang="ts">
  import { goto } from "$app/navigation";
  import authStore, { clearAuth, initializeAuth, type AuthState } from "$lib/stores/authStore";
  import { onMount } from "svelte";

  let currentUserState: AuthState;

  authStore.subscribe(value => {
    currentUserState = value;
  });

  onMount(() => {
    // Initialize auth state from localStorage when the component mounts
    // This ensures that if the user was previously logged in and refreshes the page,
    // their session is restored from localStorage.
    initializeAuth();
  });

  function handleLogout() {
    clearAuth();
    goto("/login");
  }
</script>

<nav class="bg-gray-800 text-white p-4">
  <div class="container mx-auto flex justify-between items-center">
    <a href="/" class="text-xl font-bold">ReceiptMaster</a>
    <div>
      {#if currentUserState?.isAuthenticated}
        <a href="/dashboard" class="px-3 py-2 rounded hover:bg-gray-700">Dashboard</a>
        {#if currentUserState.user?.role === "admin"}
          <a href="/admin/dashboard" class="px-3 py-2 rounded hover:bg-gray-700">Admin</a>
        {/if}
        <button on:click={handleLogout} class="px-3 py-2 rounded hover:bg-gray-700">Logout</button>
        <span class="ml-4">Welcome, {currentUserState.user?.name}!</span>
      {:else}
        <a href="/login" class="px-3 py-2 rounded hover:bg-gray-700">Login</a>
        <a href="/register" class="px-3 py-2 rounded hover:bg-gray-700">Register</a>
      {/if}
    </div>
  </div>
</nav>

