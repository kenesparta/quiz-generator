<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  let username = '';
  let password = '';
  let error = '';
  let usernameInput: HTMLInputElement;

  onMount(() => {
    usernameInput.focus();
  });

  async function handleSubmit() {
    error = '';

    if (!username || !password) {
      error = 'Please enter both username and password';
      return;
    }

    try {
      // Here you would typically make an API call to your authentication endpoint
      // For example:
      // const response = await fetch('/api/login', {
      //     method: 'POST',
      //     body: JSON.stringify({ username, password }),
      //     headers: {
      //         'Content-Type': 'application/json'
      //     }
      // });

      // if (response.ok) {
      //     const data = await response.json();
      //     // Set auth token or user data in localStorage/sessionStorage
      //     goto('/dashboard');
      // } else {
      //     error = 'Invalid username or password';
      // }

      // For now, let's use a mock successful login
      console.log('Login attempt with:', { username, password });

      // Simulate successful login
      goto('/dashboard');
    } catch (err) {
      console.error('Login error:', err);
      error = 'An error occurred during login';
    }
  }
</script>

<div class="page-container">
    <div class="login-container">
        <h1>Login</h1>

        <form on:submit|preventDefault={handleSubmit}>
            {#if error}
                <div class="error-message">{error}</div>
            {/if}

            <div class="form-group">
                <label for="username">Username</label>
                <input
                        type="text"
                        id="username"
                        bind:value={username}
                        bind:this={usernameInput}
                        placeholder="Enter your username"
                        autocomplete="username"
                />
            </div>

            <div class="form-group">
                <label for="password">Password</label>
                <input
                        type="password"
                        id="password"
                        bind:value={password}
                        placeholder="Enter your password"
                        autocomplete="current-password"
                />
            </div>

            <button type="submit" class="login-button">Log In</button>
        </form>
    </div>
</div>

<style>
    .page-container {
        display: grid;
        place-items: center;
        min-height: 100vh;
        width: 100%;
        padding: 1rem;
        box-sizing: border-box;
        background-color: #f5f5f5;
    }

    .login-container {
        width: 100%;
        max-width: 400px;
        padding: 2rem;
        border-radius: 8px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        background-color: #fff;
    }

    h1 {
        text-align: center;
        margin-bottom: 1.5rem;
        color: #333;
    }

    .form-group {
        margin-bottom: 1rem;
    }

    label {
        display: block;
        margin-bottom: 0.5rem;
        font-weight: 500;
    }

    input {
        width: 100%;
        padding: 0.75rem;
        border: 1px solid #ddd;
        border-radius: 4px;
        font-size: 1rem;
        box-sizing: border-box;
    }

    input:focus {
        outline: none;
        border-color: #4d90fe;
        box-shadow: 0 0 0 2px rgba(77, 144, 254, 0.2);
    }

    .login-button {
        width: 100%;
        padding: 0.75rem;
        background-color: #4d90fe;
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 1rem;
        font-weight: 500;
        cursor: pointer;
        margin-top: 1rem;
    }

    .login-button:hover {
        background-color: #357ae8;
    }

    .error-message {
        background-color: #ffebee;
        color: #d32f2f;
        padding: 0.75rem;
        border-radius: 4px;
        margin-bottom: 1rem;
        font-size: 0.9rem;
    }
</style>