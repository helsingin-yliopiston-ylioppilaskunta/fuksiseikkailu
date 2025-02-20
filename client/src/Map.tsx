import { useRef, useEffect, useState } from 'react'
import L from "leaflet";
import "leaflet/dist/leaflet.css";

function Map() {
    // const [count, setCount] = useState(0)
    const [points, setPoints] = useState([]);

    const mapContainerRef = useRef<HTMLDivElement>(null);
    const mapRef = useRef<L.Map | null>(null);

    useEffect(() => {
        if (!mapRef.current && mapContainerRef.current) {
            // Initialize map only if it's not done before
            mapRef.current = L.map(mapContainerRef.current).setView([60.16936416230424, 24.94024164353307], 13);

            // Add tile layer
            L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
                maxZoom: 19,
                attribution: '&copy; OpenStreetMap contributors'
            }).addTo(mapRef.current);

            //Add a marker
            L.marker([60.16936416230424, 24.94024164353307])
                .addTo(mapRef.current)
                .bindPopup('A pretty CSS3 popup.<br> Easily customizable.')
                .openPopup();

            mapRef.current.on("click", (e) => {
                setPoints((prevPoints) => [...prevPoints, e.latlng]);
            })
        }

        return () => {
            if (mapRef.current) {
                mapRef.current.off("click", () => {});
                mapRef.current.remove();
                mapRef.current = null;
            }
        }
    }, []);

    useEffect(() => {
        console.log(points);
        if (points.length >= 4) {
            L.polygon(points).addTo(mapRef.current);
        }
    }, [points])

    return <div ref={mapContainerRef} style={{ height: "800px", width: "100%" }} />;
    };

export default Map
