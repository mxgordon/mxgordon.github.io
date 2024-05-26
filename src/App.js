import './App.css';

function App() {
  return (
    <div className="App bg-cyan-50">
      <header className="p-2 px-6 w-screen flex justify-between">
        <div className="flex flex-row space-x-10 items-center">
        </div>
        <div className="flex flex-row space-x-10 items-center">
          <a className="text-xl font-semibold animated-underline after:bg-red-500" href="/#about">About</a>
          <a className="text-xl font-semibold animated-underline after:bg-red-500" href="/#projects">Projects</a>
          <a className="text-xl font-semibold animated-underline after:bg-red-500" href="/#experience">Experience</a>
        </div>
      </header>
      <div className="max-w-screen-xl m-auto my-3 px-8">
        <div className="title flex flex-col">
          <h1 className="text-4xl font-bold">
            Hello! I'm <span className="text-red-800">Max Gordon</span>
          </h1>
          <p className="my-1 font-semibold">20 Year-old computer science BS/MS student at WPI</p>
        </div>
        <br/>
        <div id="abou">
          <h2 className="text-2xl font-bold">About Me</h2>
          <p className="text-lg mb-3">
            Since high school I have been interested in programming and computer science as a whole. Since learning to program I have explored my interests greatly in robotics, deep learning, web development, and embedded programming. However, I discovered my deep interest in the intersection between technology and finance, and have since been pursuing it.
          </p>
          <p className="text-lg mb-3">
            Outside of my academic interests, in my free time I play on WPI's club ice hockey team, I shoot film photography, and I enjoy tinkering and using my 3D printer.
          </p>
        </div>
        <div id="projects"></div>
        <div id="experience"></div>
      </div>

    </div>
  );
}

export default App;
