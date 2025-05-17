<script lang="ts">
  interface Postulante {
    id: number;
    nombre: string;
    apellido: string;
    email: string;
    telefono: string;
  }

  const postulantesList = () => {

  }

  let postulantes: Postulante[] = [
    {
      id: 1,
      nombre: 'Juan',
      apellido: 'Pérez',
      email: 'juan.perez@example.com',
      telefono: '555-1234'
    },
    {
      id: 2,
      nombre: 'María',
      apellido: 'González',
      email: 'maria.gonzalez@example.com',
      telefono: '555-5678'
    }
  ];

  let showModal = false;
  let modalMode: 'add' | 'edit' = 'add';

  let currentPostulante: Omit<Postulante, 'id'> & { id?: number } = {
    nombre: '',
    apellido: '',
    email: '',
    telefono: ''
  };

  // Validation errors
  let errors = {
    nombre: '',
    apellido: '',
    email: '',
    telefono: ''
  };

  function openAddModal(): void {
    modalMode = 'add';
    currentPostulante = {
      nombre: '',
      apellido: '',
      email: '',
      telefono: ''
    };
    clearErrors();
    showModal = true;
  }

  function openEditModal(postulante: Postulante): void {
    modalMode = 'edit';
    currentPostulante = { ...postulante };
    clearErrors();
    showModal = true;
  }

  // Close the modal
  function closeModal(): void {
    showModal = false;
  }

  // Clear all validation errors
  function clearErrors(): void {
    errors = {
      nombre: '',
      apellido: '',
      email: '',
      telefono: ''
    };
  }

  // Validate the form
  function validateForm(): boolean {
    let isValid = true;
    clearErrors();

    // Check required fields
    if (!currentPostulante.nombre.trim()) {
      errors.nombre = 'El nombre es requerido';
      isValid = false;
    }

    if (!currentPostulante.apellido.trim()) {
      errors.apellido = 'El apellido es requerido';
      isValid = false;
    }

    // Validate email format
    if (!currentPostulante.email.trim()) {
      errors.email = 'El email es requerido';
      isValid = false;
    } else if (!/^\S+@\S+\.\S+$/.test(currentPostulante.email)) {
      errors.email = 'El formato del email es inválido';
      isValid = false;
    }

    // Validate phone format - simple check
    if (!currentPostulante.telefono.trim()) {
      errors.telefono = 'El teléfono es requerido';
      isValid = false;
    } else if (!/^[0-9\-\+\s]+$/.test(currentPostulante.telefono)) {
      errors.telefono = 'El formato del teléfono es inválido';
      isValid = false;
    }

    return isValid;
  }

  // Save postulante (add or update)
  function savePostulante(): void {
    if (!validateForm()) return;

    if (modalMode === 'add') {
      // Generate a simple ID (in a real app, this would come from the backend)
      const newId = postulantes.length > 0
        ? Math.max(...postulantes.map(p => p.id)) + 1
        : 1;

      // Add the new postulante to the array
      postulantes = [
        ...postulantes,
        {
          id: newId,
          nombre: currentPostulante.nombre,
          apellido: currentPostulante.apellido,
          email: currentPostulante.email,
          telefono: currentPostulante.telefono
        }
      ];
    } else if (modalMode === 'edit' && currentPostulante.id) {
      // Update existing postulante
      postulantes = postulantes.map(p =>
        p.id === currentPostulante.id
          ? { ...currentPostulante as Postulante }
          : p
      );
    }

    // Close modal after save
    closeModal();
  }

  // Delete postulante
  function deletePostulante(id: number): void {
    if (confirm('¿Está seguro que desea eliminar este postulante?')) {
      postulantes = postulantes.filter(p => p.id !== id);
    }
  }
</script>

<h1>Postulante</h1>

<div class="container">
    <div class="header-actions">
        <button class="action-button" on:click={openAddModal}>
            Agregar Postulante
        </button>
    </div>

    {#if postulantes.length > 0}
        <table class="postulantes-table">
            <thead>
            <tr>
                <th>ID</th>
                <th>Nombre</th>
                <th>Apellido</th>
                <th>Email</th>
                <th>Teléfono</th>
                <th>Acciones</th>
            </tr>
            </thead>
            <tbody>
            {#each postulantes as postulante (postulante.id)}
                <tr>
                    <td>{postulante.id}</td>
                    <td>{postulante.nombre}</td>
                    <td>{postulante.apellido}</td>
                    <td>{postulante.email}</td>
                    <td>{postulante.telefono}</td>
                    <td class="actions">
                        <button class="edit-button" on:click={() => openEditModal(postulante)}>Editar</button>
                        <button class="delete-button" on:click={() => deletePostulante(postulante.id)}>Eliminar</button>
                    </td>
                </tr>
            {/each}
            </tbody>
        </table>
    {:else}
        <div class="no-data">
            No hay postulantes registrados
        </div>
    {/if}

    <!-- Modal Component -->
    {#if showModal}
        <div class="modal-backdrop">
            <div class="modal">
                <div class="modal-header">
                    <h2>{modalMode === 'add' ? 'Agregar Postulante' : 'Editar Postulante'}</h2>
                    <button class="close-button" on:click={closeModal}>&times;</button>
                </div>
                <div class="modal-body">
                    <form on:submit|preventDefault={savePostulante}>
                        <div class="form-row">
                            <div class="form-group">
                                <label for="nombre">Nombre</label>
                                <input
                                        type="text"
                                        id="nombre"
                                        bind:value={currentPostulante.nombre}
                                />
                                {#if errors.nombre}
                                    <div class="error-message">{errors.nombre}</div>
                                {/if}
                            </div>

                            <div class="form-group">
                                <label for="apellido">Apellido</label>
                                <input
                                        type="text"
                                        id="apellido"
                                        bind:value={currentPostulante.apellido}
                                />
                                {#if errors.apellido}
                                    <div class="error-message">{errors.apellido}</div>
                                {/if}
                            </div>
                        </div>

                        <div class="form-row">
                            <div class="form-group">
                                <label for="email">Email</label>
                                <input
                                        type="email"
                                        id="email"
                                        bind:value={currentPostulante.email}
                                />
                                {#if errors.email}
                                    <div class="error-message">{errors.email}</div>
                                {/if}
                            </div>

                            <div class="form-group">
                                <label for="telefono">Teléfono</label>
                                <input
                                        type="tel"
                                        id="telefono"
                                        bind:value={currentPostulante.telefono}
                                />
                                {#if errors.telefono}
                                    <div class="error-message">{errors.telefono}</div>
                                {/if}
                            </div>
                        </div>

                        <div class="form-actions">
                            <button type="button" on:click={closeModal} class="cancel-button">
                                Cancelar
                            </button>
                            <button type="submit" class="save-button">
                                Guardar
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    {/if}
</div>


<style>
    .container {
        margin: 20px;
        display: grid;
        grid-template-rows: auto 1fr;
        gap: 20px;
    }

    .header-actions {
        display: grid;
        justify-content: end;
    }

    .action-button {
        background-color: #4CAF50;
        color: white;
        padding: 10px 15px;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-weight: bold;
    }

    .form-row {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 20px;
        margin-bottom: 15px;
    }

    .form-group {
        display: grid;
        grid-template-rows: auto auto auto;
        gap: 5px;
    }

    .form-group label {
        font-weight: bold;
    }

    .form-group input {
        width: 100%;
        padding: 8px;
        border: 1px solid #ddd;
        border-radius: 4px;
        box-sizing: border-box;
    }

    .error-message {
        color: #f44336;
        font-size: 0.8rem;
    }

    .form-actions {
        display: grid;
        grid-template-columns: auto auto;
        justify-content: end;
        gap: 10px;
        margin-top: 20px;
    }

    .cancel-button {
        background-color: #f5f5f5;
        border: 1px solid #ddd;
        padding: 8px 15px;
        border-radius: 4px;
        cursor: pointer;
    }

    .save-button {
        background-color: #4CAF50;
        color: white;
        border: none;
        padding: 8px 15px;
        border-radius: 4px;
        cursor: pointer;
    }

    .postulantes-table {
        width: 100%;
        border-collapse: collapse;
        display: grid;
        grid-template-rows: auto 1fr;
    }

    .postulantes-table thead {
        display: grid;
    }

    .postulantes-table tbody {
        display: grid;
    }

    .postulantes-table tr {
        display: grid;
        grid-template-columns: 0.5fr 1fr 1fr 1.5fr 1fr 1fr;
        border-bottom: 1px solid #ddd;
    }

    .postulantes-table th,
    .postulantes-table td {
        padding: 12px;
        text-align: left;
    }

    .postulantes-table th {
        background-color: #f2f2f2;
        font-weight: bold;
    }

    .postulantes-table tr:hover {
        background-color: #f5f5f5;
    }

    .actions {
        display: grid;
        grid-template-columns: auto auto;
        gap: 8px;
    }

    .edit-button {
        background-color: #2196F3;
        color: white;
        border: none;
        padding: 5px 10px;
        border-radius: 4px;
        cursor: pointer;
    }

    .delete-button {
        background-color: #f44336;
        color: white;
        border: none;
        padding: 5px 10px;
        border-radius: 4px;
        cursor: pointer;
    }

    .no-data {
        text-align: center;
        padding: 20px;
        background-color: #f9f9f9;
        border-radius: 4px;
        color: #666;
    }

    /* Modal styles */
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: rgba(0, 0, 0, 0.5);
        display: grid;
        justify-content: center;
        align-content: start;
        z-index: 1000;
        padding-top: 50px;
    }

    .modal {
        display: grid;
        grid-template-rows: auto 1fr;
        background-color: white;
        border-radius: 8px;
        width: 90%;
        max-width: 600px;
        max-height: 90vh;
        overflow-y: auto;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
    }

    .modal-header {
        display: grid;
        grid-template-columns: 1fr auto;
        align-items: center;
        padding: 15px 20px;
        border-bottom: 1px solid #eee;
    }

    .modal-header h2 {
        margin: 0;
        font-size: 1.5rem;
    }

    .close-button {
        background: none;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
        padding: 0;
        line-height: 1;
    }

    .modal-body {
        padding: 20px;
    }
</style>