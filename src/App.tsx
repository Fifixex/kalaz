import "./App.css";
import { KeyCap } from "./components/KeyCap";
import { Siderbar } from "./components/Sidebar";

const KEYSOUNDS = [
  {
    id: 1,
    name: 'Cherry MX Blue',
    description: '',
    color: 'blue'
  },
  {
    id: 2,
    name: 'Greenberry MX',
    description: '',
    color: 'green'
  },
  {
    id: 3,
    name: 'Cream RVRB',
    description: '',
    color: 'pink'
  },
]

function App() {
  return (
    <>
      <Siderbar />
      <main className="main-content">
        <div className="content-wrapper">
          <div className="background" />
          <section className="sounds">
            <h2>Keyboard Sounds</h2>
            <p>Debajo se encuentra el catálogo de sonidos en el cual con tus Coins puedes comprarlos.</p>

            <div className="grid">
              {
                KEYSOUNDS.map((sound, i) => (
                  <div className="card" key={i}>
                    <div className="icon">
                      <KeyCap color={sound.color} />
                    </div>
                    <h2>{sound.name}</h2>
                    <p>{sound.description}</p>
                    <button className="button">Usar</button>
                  </div>
                ))
              }
            </div>

          </section>
        </div>
      </main>
    </>
  );
}

export default App;
