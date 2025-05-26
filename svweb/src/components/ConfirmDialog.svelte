<!-- src/components/ConfirmDialog.svelte -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let title = 'Confirmar';
  export let message = '¿Está seguro?';
  export let confirmText = 'Confirmar';
  export let cancelText = 'Cancelar';
  export let type: 'danger' | 'warning' | 'info' = 'danger';

  const dispatch = createEventDispatcher<{
    confirm: void;
    cancel: void;
  }>();

  function confirm() {
    dispatch('confirm');
  }

  function cancel() {
    dispatch('cancel');
  }

  // Close on Escape key
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      cancel();
    }
  }
</script>

<style>
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
    }

    .confirm-dialog {
        background-color: white;
        padding: 24px;
        border-radius: 4px;
        width: 90%;
        max-width: 400px;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
        border: 2px solid black;
        animation: pop-in 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
    }

    .confirm-dialog.danger {
        border-top: 6px solid var(--color-accent-error);
    }

    .confirm-dialog.warning {
        border-top: 6px solid var(--color-accent-warning);
    }

    .confirm-dialog.info {
        border-top: 6px solid var(--color-primary);
    }

    h2 {
        margin: 0 0 16px 0;
        font-size: 1.5rem;
    }

    p {
        margin: 0 0 24px 0;
        color: #555;
    }

    .actions {
        display: flex;
        justify-content: flex-end;
        gap: 12px;
    }

    button {
        padding: 10px 16px;
        border: 2px solid black;
        font-weight: bold;
        cursor: pointer;
    }

    .cancel-button {
        background-color: #f5f5f5;
    }

    .confirm-button {
        background-color: var(--color-accent-error);
        color: white;
    }

    @keyframes pop-in {
        0% {
            transform: scale(0.8);
            opacity: 0;
        }
        100% {
            transform: scale(1);
            opacity: 1;
        }
    }
</style>

<svelte:window on:keydown={handleKeydown} />

<div class="modal-backdrop" on:click={cancel}>
    <div class="confirm-dialog {type}" on:click|stopPropagation>
        <h2>{title}</h2>
        <p>{message}</p>
        <div class="actions">
            <button class="cancel-button" on:click={cancel}>{cancelText}</button>
            <button class="confirm-button" on:click={confirm}>{confirmText}</button>
        </div>
    </div>
</div>