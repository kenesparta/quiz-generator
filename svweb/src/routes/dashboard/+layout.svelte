<script lang="ts">
  import { goto } from '$app/navigation';

  function navigateTo(path: string) {
    goto(path);
  }
</script>

<div class="dashboard-container">
    <nav class="menu">
        <ul>
            <li>
                <a href="/dashboard"
                   on:click|preventDefault={() => navigateTo('/dashboard')}>Inicio
                </a>
            </li>
            <li>
                <a href="/dashboard/postulante"
                   on:click|preventDefault={() => navigateTo('/dashboard/postulante')}>Postulante
                </a>
            </li>
            <li>
                <a href="/dashboard/evaluacion"
                   on:click|preventDefault={() => navigateTo('/dashboard/evaluacion')}>Evaluación
                </a>
            </li>
            <li>
                <a href="/dashboard/examen"
                   on:click|preventDefault={() => navigateTo('/dashboard/examen')}>Examen
                </a>
            </li>
            <li>
                <a href="/login"
                   on:click|preventDefault={() => navigateTo('/login')}>Salir
                </a>
            </li>
        </ul>
    </nav>

    <section class="content">
        <slot/>
    </section>
</div>

<style>
    .dashboard-container {
        display: grid;
        grid-template-columns: 190px 1fr;
        height: 100%;
    }

    .menu {
        background: var(--color-secondary);
        display: grid;
    }

    .content {
        padding: 20px;
    }

    ul {
        list-style: none;
        padding: 0;
    }

    a {
        text-decoration: none;
        color: #fff;
        display: block;
        transition: color 0.3s ease-in-out;
        margin: 0;
        padding: .8em;
    }

    a:hover {
        color: var(--color-accent-success-hover);
        font-weight: bold;
        border-right: 5px solid var(--color-accent-success-hover);
        text-decoration: none;
    }
</style>