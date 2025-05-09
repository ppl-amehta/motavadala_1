<script lang="ts">
  let email = 
""
;
  let password = 
""
;
  let errorMessage = 
""
;

  async function handleSubmit() {
    errorMessage = 
""
;
    // TODO: Implement actual login logic, call backend API
    console.log("Login attempt with:", email, password);
    if (email === "admin@example.com" && password === "password") {
      // Simulate successful login
      alert("Login successful! (Simulated)");
      // TODO: Redirect to dashboard or appropriate page
      // import { goto } from 
"$app/navigation"
;
      // goto("/dashboard");
    } else {
      errorMessage = "Invalid email or password. (Simulated)";
    }
  }
</script>

<div class="flex items-center justify-center min-h-screen bg-gray-100">
  <div class="px-8 py-6 mt-4 text-left bg-white shadow-lg rounded-lg sm:w-full md:w-1/2 lg:w-1/3">
    <h3 class="text-2xl font-bold text-center">Login to your account</h3>
    <form on:submit|preventDefault={handleSubmit}>
      <div class="mt-4">
        <div>
          <label class="block" for="email">Email</label>
          <input type="email" placeholder="Email" id="email" bind:value={email}
                 class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600">
        </div>
        <div class="mt-4">
          <label class="block" for="password">Password</label>
          <input type="password" placeholder="Password" id="password" bind:value={password}
                 class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600">
        </div>
        {#if errorMessage}
          <div class="mt-4 text-red-600">
            {errorMessage}
          </div>
        {/if}
        <div class="flex items-baseline justify-between">
          <button type="submit"
                  class="px-6 py-2 mt-4 text-white bg-blue-600 rounded-lg hover:bg-blue-900 w-full">Login</button>
        </div>
        <div class="mt-4 text-grey-600 text-center">
          Don't have an account? <a href="/register" class="text-blue-600 hover:underline">Register here</a>
        </div>
      </div>
    </form>
  </div>
</div>

