<script lang="ts">
  import { SquarePen, Trash2 } from "lucide-svelte";

  export let data
  const { postulantes } = data


  interface Postulante {
    documento: string;
    nombre_completo: string;
    fecha_nacimiento: string;
    grado_instruccion: string;
    genero: string;
  }

  interface PostulanteDTO {
    nombre: string;
    primerApellido: string;
    segundoApellido: string;
    documento: string;
    fechaNacimiento: string;
    genero: string;
    gradoInstruccion: string;
  }

  // let postulantes: Postulante[] = [
  //   {
  //     id: 1,
  //     nombre: 'Juan',
  //     apellido: 'Pérez',
  //     email: 'juan.perez@example.com',
  //     telefono: '555-1234'
  //   },
  //   {
  //     id: 2,
  //     nombre: 'María',
  //     apellido: 'González',
  //     email: 'maria.gonzalez@example.com',
  //     telefono: '555-5678'
  //   }
  // ];

  let showModal = false;
  let modalMode: 'add' | 'edit' = 'add';

  let currentPostulante: Omit<PostulanteDTO, 'id'> & { id?: number } = {
    nombre: '',
    primerApellido: '',
    segundoApellido: '',
    documento: '',
    nombreCompleto: '',
    fecha_nacimiento: '',
    grado_instruccion: '',
    genero: '',
  };

  // Validation errors
  let errors = {
    nombre: '',
    apellido: '',
    email: '',
    telefono: '',
    genero: ''
  };

  function openAddModal(): void {
    modalMode = 'add';
    currentPostulante = {
      nombre: '',
      primerApellido: '',
      segundoApellido: '',
      documento: '',
      nombreCompleto: '',
      fecha_nacimiento: '',
      grado_instruccion: '',
      genero: '',
    };
    clearErrors();
    showModal = true;
  }

  function openEditModal(postulante: PostulanteDTO): void {
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

    // // Check required fields
    // if (!currentPostulante.nombre.trim()) {
    //   errors.nombre = 'El nombre es requerido';
    //   isValid = false;
    // }
    //
    // if (!currentPostulante.apellido.trim()) {
    //   errors.apellido = 'El apellido es requerido';
    //   isValid = false;
    // }
    //
    // // Validate email format
    // if (!currentPostulante.email.trim()) {
    //   errors.email = 'El email es requerido';
    //   isValid = false;
    // } else if (!/^\S+@\S+\.\S+$/.test(currentPostulante.email)) {
    //   errors.email = 'El formato del email es inválido';
    //   isValid = false;
    // }
    //
    // // Validate phone format - simple check
    // if (!currentPostulante.telefono.trim()) {
    //   errors.telefono = 'El teléfono es requerido';
    //   isValid = false;
    // } else if (!/^[0-9\-\+\s]+$/.test(currentPostulante.telefono)) {
    //   errors.telefono = 'El formato del teléfono es inválido';
    //   isValid = false;
    // }

    // {
    //   "documento": "99009988",
    //   "nombre": "Leonardo",
    //   "apellido_paterno": "Chavez",
    //   "apellido_materno": "Espinoza",
    //   "fecha_nacimiento": "1990-01-01",
    //   "grado_instruccion": "superior",
    //   "genero": "masculino"
    // }

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
      // postulantes = [
      //   ...postulantes,
      //   {
      //     id: newId,
      //     nombre: currentPostulante.nombre,
      //     apellido: currentPostulante.apellido,
      //     email: currentPostulante.email,
      //     telefono: currentPostulante.telefono
      //   }
      // ];
    } else if (modalMode === 'edit' && currentPostulante.id) {
      // Update existing postulante
      // postulantes = postulantes.map(p =>
      //   p.id === currentPostulante.id
      //     ? { ...currentPostulante as Postulante }
      //     : p
      // );
    }

    // Close modal after save
    closeModal();
  }

  // Delete postulante
  function deletePostulante(id: number): void {
    if (confirm('¿Está seguro que desea eliminar este postulante?')) {
      // postulantes = postulantes.filter(p => p.id !== id);
    }
  }
</script>

<div class="container">
    <section class="title-section">
        <h1 class="main__title">Postulante</h1>
        <div class="main__search">
            <input type="texte", placeholder="Buscar"/>
        </div>
        <div class="header-actions">
            <button class="action-button" on:click={openAddModal}>
                Nuevo
            </button>
        </div>
    </section>

    {#if postulantes.length > 0}
        <table class="postulantes-table">
            <thead>
            <tr>
                <th>Documento</th>
                <th>Nombre Completo</th>
                <th>Fecha Nacimiento</th>
                <th>Grado Instruccion</th>
                <th>Acciones</th>
            </tr>
            </thead>
            <tbody>
            {#each postulantes as postulante (postulante.id)}
                <tr>
                    <td>{postulante.documento}</td>
                    <td>{postulante.nombre_completo}</td>
                    <td>{postulante.fecha_nacimiento}</td>
                    <td>{postulante.grado_instruccion}</td>
                    <td class="actions">
                        <button class="edit-button"
                                on:click={() => openEditModal(postulante)}><SquarePen size="15" /></button>
                        <button class="delete-button" on:click={() => deletePostulante(postulante.id)}><Trash2
                                size="15" /></button>
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
                            <label for="primerApellido">Primer Apellido</label>
                            <input
                                    type="text"
                                    id="primerApellido"
                                    bind:value={currentPostulante.primerApellido}
                            />
                            {#if errors.apellido}
                                <div class="error-message">{errors.apellido}</div>
                            {/if}
                        </div>

                        <div class="form-group">
                            <label for="segundoApellido">Segundo Apellido</label>
                            <input
                                    type="text"
                                    id="segundoApellido"
                                    bind:value={currentPostulante.segundoApellido}
                            />
                            {#if errors.apellido}
                                <div class="error-message">{errors.apellido}</div>
                            {/if}
                        </div>

                        <div class="form-group">
                            <label for="fechaNacimiento">Fecha de Nacimiento</label>
                            <input
                                    type="date"
                                    id="fechaNacimiento"
                                    bind:value={currentPostulante.fechaNacimiento}
                            />
                            {#if errors.email}
                                <div class="error-message">{errors.email}</div>
                            {/if}
                        </div>

                        <div class="form-group">
                            <label for="genero">Género</label>
                            <select
                                    id="genero"
                                    bind:value={currentPostulante.genero}
                                    class="form-select"
                            >
                                <option value="">Seleccione un género</option>
                                <option value="Masculino">Masculino</option>
                                <option value="Femenino">Femenino</option>
                                <option value="NoBinario">No Binario</option>
                            </select>
                            {#if errors.genero}
                                <div class="error-message">{errors.genero}</div>
                            {/if}
                        </div>

                        <div class="form-group">
                            <label for="instruccion">Instrucción</label>
                            <select
                                    id="instruccion"
                                    bind:value={currentPostulante.gradoInstruccion}
                                    class="form-select"
                            >
                                <option value="">Seleccione un género</option>
                                <option value="Ninguno">Ninguno</option>
                                <option value="Primaria">Primaria</option>
                                <option value="Secundaria">Secundaria</option>
                                <option value="Superior">Superior</option>
                                <option value="Postgrado">Postgrado</option>
                            </select>
                            {#if errors.genero}
                                <div class="error-message">{errors.genero}</div>
                            {/if}
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
        display: grid;
        grid-template-rows: auto 1fr;
        gap: 20px;
    }

    .title-section {
        display: grid;
        grid-template-columns: 3fr 1fr auto;
        gap: 10px;
        align-items: center;
    }

    .main__title {
        margin: 0;
        padding: 0;
        font-size: 1.5rem;
        color: #444661;
    }

    .main__search input {
        width: 100%;
        box-sizing: border-box;
        border: 2px solid #aaa;
        padding: 10px;
        outline: none;
        transition: border-color 0.3s ease;
    }

    .main__search input:focus {
        border-color: #1187ff;
        box-shadow: 0 0 5px rgba(33, 150, 243, 0.5);
    }

    .header-actions {

    }

    .action-button {
        background-color: #50E100;
        padding: 10px 15px;
        border: 2px solid #000;
        cursor: pointer;
        font-weight: bold;
    }

    .form-group {
        margin-bottom: 15px;
    }

    .form-group label {
        font-weight: bold;
        font-size: 0.8rem;
    }

    .form-group input {
        width: 100%;
        padding: 8px;
        border: 2px solid #aaa;
        box-sizing: border-box;
        margin-top: 2px;
        outline: none;
        transition: border-color 0.3s ease;
    }

    .form-group input:focus {
        border-color: #1187ff;
        box-shadow: 0 0 5px rgba(33, 150, 243, 0.5);
    }


    .form-group .form-select {
        width: 100%;
        padding: 8px;
        box-sizing: border-box;
        border: 2px solid #aaa;
        margin-top: 2px;
        outline: none;
        transition: border-color 0.3s ease;
    }

    .form-group .form-select:focus {
        border-color: #1187ff;
        box-shadow: 0 0 5px rgba(33, 150, 243, 0.5);
    }

    .error-message {
        color: #E04100;
        font-size: 0.8rem;
    }

    .form-actions {
        display: grid;
        grid-template-columns: auto auto;
        /*justify-content: end;*/
        gap: 10px;
        margin-top: 20px;
    }

    .cancel-button {
        background-color: #E04100;
        padding: 8px 15px;
        cursor: pointer;
        border: 2px solid #000;
        color: white;
        font-weight: bold;
    }

    .save-button {
        background-color: #50E100;
        border: 2px solid #000;
        padding: 8px 15px;
        cursor: pointer;
        font-weight: bold;
    }

    .postulantes-table {
        width: 100%;
        border-collapse: collapse;
    }

    .postulantes-table tr {
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
        /*display: grid;*/
        /*grid-template-columns: auto auto;*/
        /*gap: 8px;*/
    }

    .edit-button {
        background-color: #0014E0;
        color: white;
        cursor: pointer;
        border: 2px solid #000;
        padding: 6px 5px 3px 5px;
        align-content: center;
    }

    .delete-button {
        background-color: #E04100;
        color: white;
        cursor: pointer;
        padding: 6px 5px 3px 5px;
        border: 2px solid #000;
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
        width: 90%;
        max-width: 700px;
        max-height: 90vh;
        overflow-y: auto;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
        border: 2px solid #000;
    }

    .modal-header {
        display: grid;
        grid-template-columns: 1fr auto;
        border-bottom: 2px solid #000;
        align-items: center;
        background: #ddd;
    }

    .modal-header h2 {
        margin: 0;
        padding: 0;
        font-size: 1rem;
        text-align: center;
    }

    .close-button {
        background: none;
        border: none;
        font-size: 1.5rem;
        cursor: pointer;
        padding: 5px 10px;
        line-height: 1;
        background: #E04100;
        font-weight: bold;
        color: white;
        border-left: 2px solid #000;
    }

    .modal-body {
        padding: 15px;
    }
</style>