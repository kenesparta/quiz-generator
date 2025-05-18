const load = async () => {
  const postulantes = await fetch('http://localhost:3003/postulante')
  return await postulantes.json()
}
