import { fetchPostulantes } from '$lib/stores/postulante';

export const load = async () => {
  return {
    postulantes: await fetchPostulantes()
  };
};