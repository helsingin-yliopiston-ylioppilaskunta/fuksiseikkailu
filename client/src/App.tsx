// import { useState } from 'react'
import './App.css'

import L from "leaflet";

import Map from "./Map/Map";
import { CheckpointList } from "./Checkpoint/Checkpoint";


function App() {
    function handleClick(event: L.LeafletMouseEvent, map: L.Map) {
        console.log(event.latlng, map);
    }

    return (
        <div className="App">
            <h1>Fuksiseikkailu</h1>
            <Map clickCallback={handleClick} />
            <CheckpointList />
        </div>
    )
}

export default App
