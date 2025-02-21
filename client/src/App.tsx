// import { useState } from 'react'
import './App.css'

import L from "leaflet";

import Map from "./Map";

function App() {
    function handleClick(event: L.LeafletMouseEvent, map: L.Map) {
        console.log(event.latlng, map);
    }

    return (
        <div className="App">
            <h1>Fuksiseikkailu</h1>
            <p>
                Tämä on fuksiseikkailu-sovelluksen äärimmäisen testaustason julkaisu!
            </p>
            <Map clickCallback={handleClick} />
        </div>
    )
}

export default App
