import { writable } from 'svelte/store';

export interface Postulante {
  id?: string;
  documento: string;
  nombre: string;
  primer_apellido: string;
  segundo_apellido: string;
  nombre_completo?: string;
  fecha_nacimiento: string;
  grado_instruccion: string;
  genero: string;
  _links?: {
    self_: string;
    update: string;
    delete: string;
    exams: string;
    results: string;
  };
}

export interface PostulanteDTO {
  id?: string;
  nombre: string;
  primerApellido: string;
  segundoApellido: string;
  documento: string;
  fechaNacimiento: string;
  genero: string;
  gradoInstruccion: string;
  _links?: {
    self_: string;
    update: string;
    delete: string;
    exams: string;
    results: string;
  };
}

export const postulantes = writable<Postulante[]>([]);

export const fetchPostulantes = async () => {
  try {
    const response = await fetch('http://localhost:8000/postulantes');
    if (!response.ok) throw new Error('Failed to fetch postulantes');
    const data = await response.json();
    postulantes.set(data);
    return data;
  } catch (error) {
    console.error('Error fetching postulantes:', error);
    return [];
  }
};

export const addPostulante = async (postulanteData: Omit<PostulanteDTO, 'id' | '_links'>) => {
  try {
    const { v4: uuidv4 } = await import('uuid');
    const id = uuidv4();

    const newPostulante: Postulante = {
      documento: postulanteData.documento,
      nombre: postulanteData.nombre,
      primer_apellido: postulanteData.primerApellido,
      segundo_apellido: postulanteData.segundoApellido,
      fecha_nacimiento: postulanteData.fechaNacimiento,
      grado_instruccion: postulanteData.gradoInstruccion,
      genero: postulanteData.genero
    }

    const response = await fetch(`http://localhost:8000/postulantes/${id}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(newPostulante),
    });

    if (!response.ok) throw new Error('Failed to create postulante');
    newPostulante.id = id
    postulantes.update(items => [...items, newPostulante]);

    return newPostulante;
  } catch (error) {
    console.error('Error creating postulante:', error);
    throw error;
  }
};

export const updatePostulante = async (postulanteData: PostulanteDTO) => {
  try {
    const updatedPostulante: Postulante = {
      documento: postulanteData.documento,
      nombre: postulanteData.nombre,
      primer_apellido: postulanteData.primerApellido,
      segundo_apellido: postulanteData.segundoApellido,
      fecha_nacimiento: postulanteData.fechaNacimiento,
      grado_instruccion: postulanteData.gradoInstruccion,
      genero: postulanteData.genero
    }
    const response = await fetch(`http://localhost:8000/postulantes/${postulanteData.id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(updatedPostulante),
    });

    if (!response.ok) throw new Error('Failed to update postulante');

    updatedPostulante.id = postulanteData.id
    postulantes.update(items =>
      items.map(item => item.id === updatedPostulante.id ? updatedPostulante : item)
    );

    return updatedPostulante;
  } catch (error) {
    console.error('Error updating postulante:', error);
    throw error;
  }
};

export const deletePostulante = async (id: string, deleteUrl?: string) => {
  try {
    const url = `http://localhost:8000${deleteUrl || `/postulantes/${id}`}`;

    const response = await fetch(url, {
      method: 'DELETE',
    });

    if (!response.ok) throw new Error('Failed to delete postulante');

    postulantes.update(items => items.filter(item => item.id !== id));

    return true;
  } catch (error) {
    console.error('Error deleting postulante:', error);
    throw error;
  }
};