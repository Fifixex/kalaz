import { listen } from "@tauri-apps/api/event";
import "./App.css";
import { KeyCap } from "./components/KeyCap";
import { Siderbar } from "./components/Sidebar";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

const KEYSOUNDS = [
  {
    id: 1,
    name: 'Cherry MX Blue',
    filename: 'cherry-mx-blue.ogg',
    description: '',
    color: '#ffa1a1'
  },
  {
    id: 2,
    name: 'Greenberry MX',
    filename: 'cherry-mx-blue.ogg',
    description: '',
    color: '#94ff70'
  },
  {
    id: 3,
    name: 'Cream RVRB',
    filename: 'cherry-mx-blue.ogg',
    description: '',
    color: '#ff708f'
  },
]

function App() {

  const [keys, setKey] = useState("");
  listen<string>('key_pressed', (event) => {
    setKey(event.payload)
  });

  return (
    <>
      <Siderbar />
      <main className="main-content">
        <div className="content-wrapper">
          <div className="background" />
          <section className="sounds">
            <h2>Keyboard Sounds</h2>
            <p>Debajo se encuentra el catálogo de sonidos en el cual con tus Coins puedes comprarlos.</p>
            {keys || 0}
            <div className="grid">
              {
                KEYSOUNDS.map((sound, i) => (
                  <div className="card" key={i}>
                    <div className="icon">
                      <KeyCap color={sound.color} />
                    </div>
                    <h2>{sound.name}</h2>
                    <p>{sound.description}</p>
                    <button
                      className="button"
                      onClick={() => {
                        let filename: string = sound.filename;
                        invoke("set_active_sound", { sound: filename });
                      }}
                    >Usar</button>
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
