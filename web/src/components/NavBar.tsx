import React from "react";

// Define the props interface
// interface MyComponentProps {
//   title: string;
//   count: number;
//   isActive?: boolean; // Optional prop with the ? symbol
//   onClick: (id: number) => void; // Function prop
// }
//
// // Create the component using the props interface
// const MyComponent: React.FC<MyComponentProps> = ({
//                                                    title,
//                                                    count,
//                                                    isActive = false, // Default value for optional prop
//                                                    onClick
//                                                  }) => {
//   return (
//     <div className={`component ${isActive ? 'active' : ''}`}>
//       <h2>{title}</h2>
//       <p>Count: {count}</p>
//       <button onClick={() => onClick(count)}>
//         Click me
//       </button>
//     </div>
//   );
// };

interface MenuItemProps {
  text: string;
  link: string
}

const MenuItem: React.FC<MenuItemProps> = ({ text, link }) => {
  return (
    <li>
      <a href={link} className="hover:text-black hover:bg-gray-200 dark:text-gray-200 p-2 rounded-md">
        {text}
      </a>
    </li>
  );
};
/*
* m: margin: m-4
* p: padding: p-6
* mt: margin-top
* mb: margin-bottom
* mr: margin-right
* ml: margin-left
* border
* */

const NavBar: React.FC = () => {
  return (
    <>
      <header className="bg-blue-500 text-white dark:bg-blue-900 dark:text-gray-200">
        <nav className="container mx-auto flex items-center justify-between py-4">
          <a href="#" className="text-lg font-bold">LOGO</a>
          <ul className="flex space-x-4">
            <MenuItem link="#" text="🏠"/>
            <MenuItem link="#" text="👨🏻‍💼 Postulantes"/>
            <MenuItem link="#" text="📝 Evaluaciones"/>
          </ul>
        </nav>
      </header>

      <section>
        <div className="m-4 p-6 bg-amber-200"> Boxes </div>
        <div className="m-4 p-6 bg-amber-200"> Boxes </div>
        <div className="border-4 border-red-300 p-4 m-8 rounded-3xl shadow-2xl"> Boxes </div>
      </section>

      <section className="grid grid-cols-3 gap-4">
        <div className="bg-blue-500 p-4 col-span-2"> Hello</div>
        <div className="bg-blue-500 p-4"> Hello</div>
        <div className="bg-blue-500 p-4"> Hello</div>
        <div className="bg-blue-500 p-4"> Hello</div>
        <div className="bg-blue-500 p-4"> Hello</div>
      </section>

      <div className="bg-amber-300 h-screen">
        Hello full screen
      </div>

    </>
  )
}

export { NavBar }