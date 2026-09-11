import * as React from 'react'
import { Marker, Popup, useMap } from '@vis.gl/react-maplibre'
import Supercluster from 'supercluster'
import type { PointFeature } from 'supercluster'
import { Search, X } from 'lucide-react'
import { VectorMap } from './Map'
import { cn } from '@/lib/utils'

type BBox = [number, number, number, number]

export interface Checkpoint {
    id: string
    name: string
    description?: string
    latitude: number
    longitude: number
    number?: number
    icon?: React.ReactNode
    category?: 'academic' | 'party' | 'sports' | 'start' | 'afterparty' | 'default'
    color?: string
}

interface CheckpointProperties {
    cluster: false
    checkpoint: Checkpoint
}

type CheckpointFeature = PointFeature<CheckpointProperties>

const CATEGORY_COLORS: Record<string, string> = {
    academic: 'bg-blue-400 text-black',
    party: 'bg-pink-400 text-black',
    sports: 'bg-emerald-400 text-black',
    start: 'bg-amber-400 text-black',
    afterparty: 'bg-purple-400 text-black',
    default: 'bg-blush-pop-400 text-black',
}

function CheckpointSearch({
    checkpoints,
    onSelect,
}: {
    checkpoints: Checkpoint[]
    onSelect: (checkpoint: Checkpoint) => void
}) {
    const { current: map } = useMap()
    const [query, setQuery] = React.useState('')
    const [isOpen, setIsOpen] = React.useState(false)

    const filtered = React.useMemo(() => {
        if (!query.trim()) return []
        const q = query.toLowerCase()
        return checkpoints.filter(
            (cp) =>
                cp.name.toLowerCase().includes(q) ||
                (cp.number !== undefined && cp.number.toString().includes(q))
        )
    }, [checkpoints, query])

    const handleSelect = (cp: Checkpoint) => {
        setQuery('')
        setIsOpen(false)
        onSelect(cp)

        map?.flyTo({
            center: [cp.longitude, cp.latitude],
            zoom: 16,
            speed: 1.4,
            essential: true,
        })
    }

    return (
        <div
            className="absolute top-3 left-3 z-10 w-72"
            onTouchStart={(e) => e.stopPropagation()}
        >
            <div className="relative flex items-center bg-surface-elevated shadow-md border-black border-2 backdrop-blur-sm">
                <Search className="ml-3 h-4 w-4 text-text-muted shrink-0" />
                <input
                    type="text"
                    value={query}
                    onChange={(e) => {
                        setQuery(e.target.value)
                        setIsOpen(true)
                    }}
                    onFocus={() => setIsOpen(true)}
                    placeholder="Search checkpoints..."
                    className="w-full bg-transparent px-3 py-2 text-base md:text-xs font-medium text-text-main placeholder-text-muted outline-none"
                />
            </div>

            {isOpen && filtered.length > 0 && (
                <ul className="max-h-60 overflow-auto bg-surface-elevated p-1 shadow-lg border-2 border-black backdrop-blur-sm border-t-0">
                    {filtered.map((cp) => (
                        <li key={cp.id}>
                            <button
                                type="button"
                                onClick={() => handleSelect(cp)}
                                className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-xs hover:bg-vintage-berry-100 transition-colors"
                            >
                                <span className="flex h-5 w-5 shrink-0 items-center justify-center rounded-full bg-vintage-berry-800 text-[10px] font-bold text-white [&>svg]:h-3 [&>svg]:w-3">
                                    {cp.icon ?? cp.number ?? '•'}
                                </span>
                                <span className="truncate font-medium text-text-main">{cp.name}</span>
                            </button>
                        </li>
                    ))}
                </ul>
            )}
        </div>
    )
}

function CheckpointMarker({
    checkpoint,
    longitude,
    latitude,
    showNameLabel,
    onClick,
}: {
    checkpoint: Checkpoint
    longitude: number
    latitude: number
    showNameLabel: boolean
    onClick: (e: { originalEvent: MouseEvent }) => void
}) {
    // Safely resolve category class with fallback to 'default'
    const categoryKey = checkpoint.category && CATEGORY_COLORS[checkpoint.category]
        ? checkpoint.category
        : 'default'
    const colorClass = CATEGORY_COLORS[categoryKey]

    return (
        <Marker longitude={longitude} latitude={latitude} anchor="center" onClick={onClick}>
            <div className="relative group flex flex-col items-center cursor-pointer">
                {/* Circle Pin - Center Anchored */}
                <div
                    className={cn(
                        'flex h-8 w-8 items-center justify-center rounded-full text-xs font-bold shadow-md transition-transform hover:scale-110 border-2 border-black',
                        colorClass
                    )}
                    style={checkpoint.color ? { backgroundColor: checkpoint.color } : undefined}
                >
                    {checkpoint.icon ? (
                        <div className="flex items-center justify-center [&>svg]:h-4 [&>svg]:w-4">
                            {checkpoint.icon}
                        </div>
                    ) : (
                        checkpoint.number ?? '•'
                    )}
                </div>

                {/* Text Label */}
                {showNameLabel && (
                    <span className="absolute top-full mt-1 left-1/2 -translate-x-1/2 pointer-events-none whitespace-nowrap bg-surface-elevated px-1.5 py-0.5 text-[11px] font-semibold text-text-main shadow-sm backdrop-blur-sm border-2 border-black">
                        {checkpoint.name}
                    </span>
                )}
            </div>
        </Marker>
    )
}

function ClusteredCheckpointMarkers({
    checkpoints,
    onCheckpointClick,
}: {
    checkpoints: Checkpoint[]
    onCheckpointClick?: (checkpoint: Checkpoint) => void
}) {
    const { current: map } = useMap()
    const [bounds, setBounds] = React.useState<BBox | null>(null)
    const [zoom, setZoom] = React.useState<number>(13)
    const [selectedCheckpoint, setSelectedCheckpoint] = React.useState<Checkpoint | null>(null)

    const supercluster = React.useMemo(() => {
        const SuperclusterConstructor = ((Supercluster as unknown as { default: typeof Supercluster }).default || Supercluster) as typeof Supercluster
        const sc = new SuperclusterConstructor<CheckpointProperties>({
            radius: 40,
            maxZoom: 16,
        })

        const features: CheckpointFeature[] = checkpoints.map((cp) => ({
            type: 'Feature',
            properties: { cluster: false, checkpoint: cp },
            geometry: {
                type: 'Point',
                coordinates: [cp.longitude, cp.latitude],
            },
        }))

        sc.load(features)
        return sc
    }, [checkpoints])

    const updateViewState = React.useCallback(() => {
        if (!map) return
        const mapBounds = map.getBounds()
        setBounds([
            mapBounds.getWest(),
            mapBounds.getSouth(),
            mapBounds.getEast(),
            mapBounds.getNorth(),
        ])
        setZoom(map.getZoom())
    }, [map])

    React.useEffect(() => {
        if (!map) return
        updateViewState()

        const handleMapClick = () => {
            setSelectedCheckpoint(null)
        }

        map.on('move', updateViewState)
        map.on('zoom', updateViewState)
        map.on('click', handleMapClick)

        return () => {
            map.off('move', updateViewState)
            map.off('zoom', updateViewState)
            map.off('click', handleMapClick)
        }
    }, [map, updateViewState])

    const clusters = React.useMemo(() => {
        if (!bounds) return []
        return supercluster.getClusters(bounds, Math.floor(zoom))
    }, [supercluster, bounds, zoom])

    const handleSelectCheckpoint = (cp: Checkpoint) => {
        setSelectedCheckpoint(cp)
        onCheckpointClick?.(cp)
    }

    return (
        <>
            <CheckpointSearch checkpoints={checkpoints} onSelect={handleSelectCheckpoint} />

            {clusters.map((cluster) => {
                const [longitude, latitude] = cluster.geometry.coordinates
                const properties = cluster.properties

                if (properties.cluster) {
                    const clusterId = cluster.id as number
                    const pointCount = properties.point_count

                    return (
                        <Marker
                            key={`cluster-${clusterId}`}
                            longitude={longitude}
                            latitude={latitude}
                            anchor="center"
                            onClick={(e) => {
                                e.originalEvent.stopPropagation()
                                const expansionZoom = Math.min(
                                    supercluster.getClusterExpansionZoom(clusterId),
                                    18
                                )
                                map?.flyTo({
                                    center: [longitude, latitude],
                                    zoom: expansionZoom,
                                    speed: 1.2,
                                })
                            }}
                        >
                            <div className="flex h-9 w-9 items-center justify-center rounded-full border-2 border-black bg-blush-pop-400 font-bold text-black shadow-lg transition-transform hover:scale-110 cursor-pointer">
                                {pointCount}
                            </div>
                        </Marker>
                    )
                }

                const cp = properties.checkpoint
                const isSelected = selectedCheckpoint?.id === cp.id
                const showNameLabel = zoom >= 14 && !isSelected

                return (
                    <CheckpointMarker
                        key={`checkpoint-${cp.id}-${longitude}-${latitude}`}
                        checkpoint={cp}
                        longitude={longitude}
                        latitude={latitude}
                        showNameLabel={showNameLabel}
                        onClick={(e) => {
                            e.originalEvent.stopPropagation()
                            handleSelectCheckpoint(cp)
                        }}
                    />
                )
            })}

            {selectedCheckpoint && (
                <Popup
                    longitude={selectedCheckpoint.longitude}
                    latitude={selectedCheckpoint.latitude}
                    anchor="bottom"
                    offset={18}
                    onClose={() => setSelectedCheckpoint(null)}
                    closeOnClick={true}
                    focusAfterOpen={false}
                    className="[&_.maplibregl-popup-content]:p-0 [&_.maplibregl-popup-content]:rounded-xl [&_.maplibregl-popup-content]:shadow-xl [&_.maplibregl-popup-content]:border-2 [&_.maplibregl-popup-content]:border-black [&_.maplibregl-popup-close-button]:hidden"
                >
                    <div className="relative min-w-[200px] max-w-xs p-3.5 bg-surface-elevated rounded-xl">
                        <button
                            type="button"
                            onClick={() => setSelectedCheckpoint(null)}
                            className="absolute top-2.5 right-2.5 flex h-5 w-5 items-center justify-center rounded-full text-text-muted hover:bg-blush-pop-100 hover:text-text-main transition-colors"
                        >
                            <X className="h-3.5 w-3.5" />
                        </button>

                        <div className="flex items-center gap-2.5 pr-6">
                            <span className="flex h-7 w-7 shrink-0 items-center justify-center rounded-full bg-blush-pop-400 text-xs font-bold text-black border border-black shadow-xs [&>svg]:h-3.5 [&>svg]:w-3.5">
                                {selectedCheckpoint.icon ?? selectedCheckpoint.number ?? '•'}
                            </span>
                            <h4 className="font-bold text-sm text-text-main leading-tight">
                                {selectedCheckpoint.name}
                            </h4>
                        </div>

                        {selectedCheckpoint.description && (
                            <p className="mt-2 text-xs text-text-muted leading-relaxed border-t border-black/20 pt-2">
                                {selectedCheckpoint.description}
                            </p>
                        )}
                    </div>
                </Popup>
            )}
        </>
    )
}

export function CheckpointMap({
    checkpoints,
    className,
    onCheckpointClick,
}: {
    checkpoints: Checkpoint[]
    className?: string
    onCheckpointClick?: (checkpoint: Checkpoint) => void
}) {
    return (
        <div className={cn("border-2 border-black h-full")}>
            <VectorMap className={className}>
                <ClusteredCheckpointMarkers checkpoints={checkpoints} onCheckpointClick={onCheckpointClick} />
            </VectorMap>
        </div>
    )
}
