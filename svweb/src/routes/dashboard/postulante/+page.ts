export const load = async ({ fetch }) => {
  const resp = await fetch('http://localhost:3003/postulantes');
  let postulantes = await resp.json()
  return {
    postulantes
  };
}
