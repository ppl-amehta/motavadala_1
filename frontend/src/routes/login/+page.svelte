<script lang="ts">
  import { goto } from "$app/navigation";
  import { setAuth } from "$lib/stores/authStore";

  let email = "";
  let password = "";
  let errorMessage = "";
  let isLoading = false;

  async function handleSubmit() {
    errorMessage = "";
    isLoading = true;

    try {
      const response = await fetch("http://localhost:3002/api/auth/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email, password }),
      });

      if (response.ok) {
        const loginResponse = await response.json(); // Expecting { user: User, token: string }
        // Ensure the response structure matches what setAuth expects
        if (loginResponse.user && loginResponse.token) {
          setAuth(loginResponse.user, loginResponse.token);
          console.log("Login successful, user:", loginResponse.user);
          // Redirect to dashboard or appropriate page based on role
          if (loginResponse.user.role === "admin") {
            goto("/admin/dashboard"); // Example admin dashboard route
          } else {
            goto("/dashboard");
          }
        } else {
          // Handle unexpected response structure
          console.error("Login response did not contain user and token:", loginResponse);
          errorMessage = "Login failed: Unexpected response from server.";
        }
      } else {
        const errorResult = await response.json();
        errorMessage = errorResult.message || `Login failed with status: ${response.status}`;
      }
    } catch (error) {
      console.error("Login error:", error);
      errorMessage = "An unexpected error occurred. Please try again.";
    }
    isLoading = false;
  }
</script>

<div class="flex items-center justify-center min-h-screen bg-gray-100">
  <div class="px-8 py-6 mt-4 text-left bg-white shadow-lg rounded-lg sm:w-full md:w-1/2 lg:w-1/3">
    <h3 class="text-2xl font-bold text-center">Login to your account</h3>
    <form on:submit|preventDefault={handleSubmit}>
      <div class="mt-4">
        <div>
          <label class="block" for="email">Email</label>
          <input type="email" placeholder="Email" id="email" bind:value={email} required
                 class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600">
        </div>
        <div class="mt-4">
          <label class="block" for="password">Password</label>
          <input type="password" placeholder="Password" id="password" bind:value={password} required
                 class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600">
        </div>
        {#if errorMessage}
          <div class="mt-4 text-red-600">
            {errorMessage}
          </div>
        {/if}
        <div class="flex items-baseline justify-between">
          <button type="submit" disabled={isLoading}
                  class="px-6 py-2 mt-4 text-white bg-blue-600 rounded-lg hover:bg-blue-900 w-full disabled:opacity-50">
            {#if isLoading}Logging in...{:else}Login{/if}
          </button>
        </div>
        <div class="mt-4 text-grey-600 text-center">
          Don\'t have an account? <a href="/register" class="text-blue-600 hover:underline">Register here</a>
        </div>
      </div>
    </form>
  </div>
</div>

