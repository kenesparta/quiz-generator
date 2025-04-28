export const PostulanteList = () => {
  return (
    <>
      <h1>Lista de los Postulantes</h1>
      <div className="overflow-x-auto">
        <table className="min-w-full bg-white border border-gray-200">
          <thead>
          <tr className="bg-gray-50">
            <th className="py-2 px-4 border-b text-left text-sm font-semibold text-gray-700">Name</th>
            <th className="py-2 px-4 border-b text-left text-sm font-semibold text-gray-700">Age</th>
            <th className="py-2 px-4 border-b text-left text-sm font-semibold text-gray-700">City</th>
          </tr>
          </thead>
          <tbody>
          <tr className="hover:bg-gray-100">
            <td className="py-2 px-4 border-b text-sm text-gray-600">Alice Smith</td>
            <td className="py-2 px-4 border-b text-sm text-gray-600">30</td>
            <td className="py-2 px-4 border-b text-sm text-gray-600">New York</td>
          </tr>
          <tr className="hover:bg-gray-100">
            <td className="py-2 px-4 border-b text-sm text-gray-600">Bob Johnson</td>
            <td className="py-2 px-4 border-b text-sm text-gray-600">25</td>
            <td className="py-2 px-4 border-b text-sm text-gray-600">London</td>
          </tr>
          <tr className="hover:bg-gray-100">
            <td className="py-2 px-4 border-b text-sm text-gray-600">Charlie Brown</td>
            <td className="py-2 px-4 border-b text-sm text-gray-600">35</td>
            <td className="py-2 px-4 border-b text-sm text-gray-600">Paris</td>
          </tr>
          <tr className="hover:bg-gray-100">
            <td className="py-2 px-4 text-sm text-gray-600">David Lee</td>
            <td className="py-2 px-4 text-sm text-gray-600">28</td>
            <td className="py-2 px-4 text-sm text-gray-600">Tokyo</td>
          </tr>
          </tbody>
        </table>
      </div>
    </>
  )
}