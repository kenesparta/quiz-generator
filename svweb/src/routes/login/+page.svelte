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
        <h1>¡Bienvenido!</h1>

        <form on:submit|preventDefault={handleSubmit}>
            {#if error}
                <div class="error-message">{error}</div>
            {/if}

            <div class="form-group">
                <label for="username">Usuario</label>
                <input
                        type="text"
                        id="username"
                        bind:value={username}
                        bind:this={usernameInput}
                        autocomplete="username"
                />
            </div>

            <div class="form-group">
                <label for="password">Contraseña</label>
                <input
                        type="password"
                        id="password"
                        bind:value={password}
                        autocomplete="current-password"
                />
            </div>

            <button type="submit" class="login-button">Ingresar</button>
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
        background-color: var(--color-secondary);
    }

    .login-container {
        width: 100%;
        max-width: 300px;
        padding: 1.5rem;
        background-color: #fff;
        border: 2px solid #000;
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
        margin-bottom: 0.2rem;
        font-weight: bold;
    }

    input {
        width: 100%;
        padding: 0.75rem;
        border: 2px solid var(--color-input-border);
        font-size: 1rem;
        box-sizing: border-box;
    }

    input:focus {
        outline: none;
        border-color: var(--color-input-focus);
        box-shadow: 0 0 0 2px var(--color-input-shadow-overlay);
    }

    .login-button {
        width: 100%;
        padding: 0.75rem;
        background-color: var(--color-primary);
        color: white;
        border: 2px solid #000;
        font-size: 1rem;
        font-weight: 500;
        cursor: pointer;
        margin-top: 1rem;
    }

    .login-button:hover {
        background-color: var(--color-primary-hover);
    }

    .error-message {
        background-color: var(--color-error-background);
        color: var(--color-error-text);
        padding: 0.75rem;
        border-radius: 4px;
        margin-bottom: 1rem;
        font-size: 0.9rem;
    }
</style>