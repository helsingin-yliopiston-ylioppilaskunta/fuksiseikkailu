import * as React from 'react'
import { Map, NavigationControl } from '@vis.gl/react-maplibre'
import * as maplibregl from 'maplibre-gl'
import { Protocol } from 'pmtiles'
import { layers, namedFlavor, type Flavor } from '@protomaps/basemaps'

import 'maplibre-gl/dist/maplibre-gl.css'
import { cn } from '@/lib/utils'

const protocol = new Protocol()
maplibregl.addProtocol('pmtiles', (request, callback) => protocol.tile(request, callback))

interface VectorMapProps {
    className?: string
    pmtilesUrl?: string
    presetTheme?: Flavor
    onClick?: (e: maplibregl.MapMouseEvent) => void
    children?: React.ReactNode
}

export function VectorMap({
    className,
    pmtilesUrl = 'https://fuksiseikkailu-maptiles.s3.fr-par.scw.cloud/helsinki.pmtiles',
    presetTheme = namedFlavor('light'),
    onClick,
    children,
}: VectorMapProps) {
    const mapStyle = React.useMemo<maplibregl.StyleSpecification>(() => {
        const baseLayers = layers('protomaps', presetTheme, { lang: 'en' })

        return {
            version: 8,
            glyphs: 'https://protomaps.github.io/basemaps-assets/fonts/{fontstack}/{range}.pbf',
            // MapLibre appends .json and .png (or @2x.png) to this base URL automatically
            sprite: 'https://protomaps.github.io/basemaps-assets/sprites/v4/light',
            sources: {
                protomaps: {
                    type: 'vector',
                    url: `pmtiles://${pmtilesUrl}`,
                },
            },
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            layers: baseLayers as any,
        }
    }, [pmtilesUrl, presetTheme])

    return (
        <div className={cn('relative h-full w-full overflow-hidden', className)}>
            <Map
                mapLib={maplibregl}
                onClick={onClick}
                initialViewState={{
                    longitude: 24.9384,
                    latitude: 60.1699,
                    zoom: 13,
                }}
                style={{ width: '100%', height: '100%' }}
                mapStyle={mapStyle}
            >
                <NavigationControl position="top-right" />
                {children}
            </Map>
        </div>
    )
}
