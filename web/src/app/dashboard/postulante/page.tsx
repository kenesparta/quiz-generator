import { ListarPostulante } from "@postulante";
import { Title } from "@shared/Title";
import Link from "next/link";

export default function PostulantePage() {
  return (
    <div className="">
      <Title name="Postulante"/>
      <Link href="/dashboard/postulante/agregar">Nuevo</Link>
      <ListarPostulante/>
    </div>
  )
}