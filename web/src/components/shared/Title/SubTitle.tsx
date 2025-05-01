interface SubTitleProps {
  name: string;
}

export const SubTitle = ({ name }: SubTitleProps) => {
  return (
    <>
      <h2 className={`text-lg text-cyan-700 font-bold`}>{name}</h2>
    </>
  )
}