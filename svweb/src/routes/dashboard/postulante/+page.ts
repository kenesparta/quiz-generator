const postulantesList = async () => {
  const response = await fetch('http://localhost:3003/postulante')
  return await response.json()
}
