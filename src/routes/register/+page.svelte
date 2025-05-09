<script lang="ts">
  let name = "";
  let email = "";
  let password = "";
  let confirmPassword = "";
  let role = "user"; // Default role
  let errorMessage = "";
  let successMessage = "";

  async function handleSubmit() {
    errorMessage = "";
    successMessage = "";
    if (password !== confirmPassword) {
      errorMessage = "Passwords do not match.";
      return;
    }
    // TODO: Implement actual registration logic, call backend API
    console.log("Registration attempt with:", name, email, password, role);
    // Simulate successful registration
    successMessage = "Registration successful! Please login. (Simulated)";
    // TODO: Optionally redirect to login page or show a success message
    // import { goto } from "$app/navigation";
    // goto("/login");
  }
</script>

<div class="flex items-center justify-center min-h-screen bg-gray-100">
  <div class="px-8 py-6 mt-4 text-left bg-white shadow-lg rounded-lg sm:w-full md:w-1/2 lg:w-1/3">
    <h3 class="text-2xl font-bold text-center">Create an account</h3>
    <form on:submit|preventDefault={handleSubmit}>
      <div class="mt-4">
        <div>
          <label class="block" for="name">Name</label>
          <input type="text" placeholder="Full Name" id="name" bind:value={name}
                 class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600">
        </div>
        <div class="mt-4">
          <label class="block" for="email">Email</label>
          <input type="email" placeholder="Email" id="email" bind:value={email}
                 class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600">
        </div>
        <div class="mt-4">
          <label class="block" for="password">Password</label>
          <input type="password" placeholder="Password" id="password" bind:value={password}
                 class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600">
        </div>
        <div class="mt-4">
          <label class="block" for="confirmPassword">Confirm Password</label>
          <input type="password" placeholder="Confirm Password" id="confirmPassword" bind:value={confirmPassword}
                 class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600">
        </div>
        <!-- Basic role selection for now, can be improved -->
        <div class="mt-4">
          <label class="block" for="role">Role</label>
          <select id="role" bind:value={role} class="w-full px-4 py-2 mt-2 border rounded-md focus:outline-none focus:ring-1 focus:ring-blue-600">
            <option value="user">User</option>
            <option value="admin">Admin</option>
          </select>
        </div>

        {#if errorMessage}
          <div class="mt-4 text-red-600">
            {errorMessage}
          </div>
        {/if}
        {#if successMessage}
          <div class="mt-4 text-green-600">
            {successMessage}
          </div>
        {/if}

        <div class="flex items-baseline justify-between">
          <button type="submit"
                  class="px-6 py-2 mt-4 text-white bg-blue-600 rounded-lg hover:bg-blue-900 w-full">Register</button>
        </div>
        <div class="mt-4 text-grey-600 text-center">
          Already have an account? <a href="/login" class="text-blue-600 hover:underline">Login here</a>
        </div>
      </div>
    </form>
  </div>
</div>

