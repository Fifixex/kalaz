import "./App.css";

function App() {
  return (
    <main className="container">
      <div className="wrapper">
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
                    <div className="button">
                      <button>Comprar</button>
                    </div>
                  </div>
                </div>
              ))
            }
          </div>

        </section>
      </div>
    </main>
  );
}

export default App;
