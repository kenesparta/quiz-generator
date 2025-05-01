interface TitleProps {
  name: string;
}

export const Title = ({ name }: TitleProps) => {
  return (
    <>
      <h1 className="text-2xl text-blue-700 font-bold">{name}</h1>
    </>
  )
}