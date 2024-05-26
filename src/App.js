import './App.css';

function App() {
  return (
    <div className="App bg-cyan-50">
      <header className="flex p-2 px-6 space-x-10 items-center w-screen flex-row place-content-end">
        <a className="text-xl font-semibold animated-underline after:bg-red-500" href="/#about">About</a>
        <a className="text-xl font-semibold animated-underline after:bg-red-500" href="/#projects">Projects</a>
        <a className="text-xl font-semibold animated-underline after:bg-red-500" href="/#experience">Experience</a>
      </header>
      <div className="max-w-screen-xl m-auto my-3">
        <div className="title flex flex-col">
          <h1 className="text-4xl font-bold">
            Hello! I'm <span className="text-red-800">Max Gordon</span>
          </h1>
          <p className="my-1 font-semibold">20 Year-old computer science BS/MS student at WPI</p>
        </div>
        <br/>
        <div id="about">
          <h2 className="text-2xl font-bold">About Me</h2>
          <p className="text-lg mb-3">
            Since high school I have been interested in programming and computer science as a whole. First, I learned C++ to write code for my Arduino. Next, Java on my FRC robotics team to control the robot. After that, Python for machine learning and other personal projects, followed by JavaScript to create web apps. Then, in college I learned C, GLSL, and R, further broadening my experience.
          </p>
          <p className="text-lg mb-3">
            While still maintaining my broad interest in computer science including robotics, graphics, deep learning, embedded programming, I have developed a particular interest in financial technology. In this field, I can utilize the cutting edge of technology, I have the opportunity to double-down on optimization, and finally the opportunity to work with mathematics like no other field.
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
