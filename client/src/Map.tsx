import { useRef, useEffect } from 'react'
import L from "leaflet";
import "leaflet/dist/leaflet.css";

import "leaflet.markercluster";
import "leaflet.markercluster/dist/MarkerCluster.css";
import "leaflet.markercluster/dist/MarkerCluster.Default.css";

interface MapProps {
    clickCallback: (e: L.LeafletMouseEvent, map: L.Map) => void;
}

const Map: React.FC<MapProps> = ({ clickCallback }) => {
    // const [points, setPoints] = useState([]);

    const mapContainerRef = useRef<HTMLDivElement>(null);
    const mapRef = useRef<L.Map | null>(null);

    const checkpoints = [
        {
            lat: 60.169571042096045,
            lng: 24.93995298815067,
            name: "A",
        },
        {
            lat: 60.169389587135456, lng: 24.94110123570203,
            name: "B",
        },
        {
            lat: 60.16973648545139, lng: 24.94120854855727,
            name: "C",
        },
        {
            lat: 60.16990206144352, lng: 24.93937765498991,
            name: "D",
        }
    ];

    useEffect(() => {
        if (!mapRef.current && mapContainerRef.current) {
            // Initialize map only if it's not done before
            mapRef.current = L.map(mapContainerRef.current).setView([60.16936416230424, 24.94024164353307], 18);

            // Add tile layer
            L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
                maxZoom: 19,
                attribution: '&copy; OpenStreetMap contributors'
            }).addTo(mapRef.current);

            // //Add a marker
            // L.marker([60.16936416230424, 24.94024164353307])
            //     .addTo(mapRef.current)
            //     .bindPopup('A pretty CSS3 popup.<br> Easily customizable.')
            //     .openPopup();
            
            const blueIcon = L.divIcon({
                className: "custom-circle",
                html: "<div style='width:20px;height:20px;border-radius:50%;background:blue;'></div>",
                iconSize: [20, 20],
                iconAnchor: [10, 0],
            })

            const redIcon = L.divIcon({
                className: "custom-circle",
                html: "<div style='width:20px;height:20px;border-radius:50%;background:red;'></div>",
                iconSize: [20, 20],
                iconAnchor: [10, 0],
            })


            const markers = checkpoints.map((point) => {
                return L.marker([point.lat, point.lng], {
                    icon: blueIcon,
                    draggable: false,
                })
                .bindTooltip(point.name, {
                        direction: "top",
                        permanent: true,
                        offset: L.point(0,0),
                    })
                .on("dragend", (e) => {
                        const newPos = e.target.getLatLng();
                        console.log("New position:", newPos);
                        e.target.setIcon(blueIcon);
                        e.target.dragging.disable();
                    })
                .on("click", (e) => {
                        e.target.setIcon(redIcon);
                        e.target.dragging.enable();
                    })
            });

            const markerCluster = L.markerClusterGroup({
                disableClusteringAtZoom: 19,
            });
            markers.forEach((marker) => {
                markerCluster.addLayer(marker);
            })
            mapRef.current.addLayer(markerCluster);

            // mapRef.current.on("click", (e) => clickCallback(e, mapRef.current));
        }

        return () => {
            if (mapRef.current) {
                mapRef.current.off("click", () => {});
                mapRef.current.remove();
                mapRef.current = null;
            }
        }
    }, [clickCallback]);

    // useEffect(() => {
    //     console.log(points);
    //     if (points.length >= 4) {
    //         L.polygon(points).addTo(mapRef.current);
    //     }
    // }, [points])

    return <div ref={mapContainerRef} style={{ height: "800px", width: "100%" }} />;
    };

export default Map
