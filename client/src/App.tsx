import { useState } from 'react'
import './App.css'

import L from "leaflet";

import Map from "./Map";

function App() {
    const [count, setCount] = useState(0)

    function handleClick(event: L.LeafletMouseEvent, map: L.Map) {
        console.log(event.latlng, map);
    }

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
            <Map clickCallback={handleClick} />
        </div>
    )
}

export default App
