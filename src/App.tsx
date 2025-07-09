import "./App.css";
import { Siderbar } from "./components/Sidebar";

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
                Array.from({ length: 5 }).fill(5).map((_, i) => (
                  <div className="card" key={i}>
                    <div className="body">
                      <h2>Cherry MX Blue</h2>
                      <p>400 coins</p>
                      <button className="button">Usar</button>
                    </div>
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
