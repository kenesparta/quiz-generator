import Link from "next/link";

export const AgregarPostulante = () => {
  return (
    <>
      <form>
        <input type="text" placeholder="Nombrs"/>
        <input type="text" placeholder="Apellido Materno"/>
        <input type="text" placeholder="Apellido paterno"/>
        <input type="text" placeholder="Documento"/>
        <input type="date" placeholder="Fecha Nacimiento"/>
        <a>Agregar</a>
        <Link href="/dashboard/postulante">Cancelar</Link>
      </form>
    </>
  )
}