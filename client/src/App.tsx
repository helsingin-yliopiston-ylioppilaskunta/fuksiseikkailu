import { useState } from 'react'
import './App.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="App">
      <h1>Fuksiseikkailu</h1>
      <div className="card">
        <p>
            Tästä se pikkuhiljaa lähtee rakentumaan!
        </p>
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
      </div>
    </div>
  )
}

export default App
