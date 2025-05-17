<script lang="ts">
  import { goto } from '$app/navigation';

  function navigateTo(path: string) {
    goto(path);
  }
</script>

<div class="dashboard-container">
    <div class="sidebar">
        <ul>
            <li><a href="/dashboard"
                   on:click|preventDefault={() => navigateTo('/dashboard')}>Dashboard</a></li>
            <li><a href="/dashboard/postulante"
                   on:click|preventDefault={() => navigateTo('/dashboard/postulante')}>Postulante
            </a></li>
        </ul>
    </div>

    <div class="content">
        <slot/>
    </div>
</div>

<style>
    .dashboard-container {
        display: flex;
        min-height: 100vh;
    }

    .sidebar {
        width: 250px;
        background-color: #f1f1f1;
        padding: 20px;
    }

    .content {
        flex-grow: 1;
        padding: 0;
    }

    ul {
        list-style: none;
        padding: 0;
    }

    li {
        margin-bottom: 10px;
    }

    a {
        text-decoration: none;
        color: #333;
    }

    a:hover {
        text-decoration: underline;
    }
</style>