import * as React from 'react'
import { Marker, Popup } from '@vis.gl/react-maplibre'
import type { MapMouseEvent } from 'maplibre-gl'
import { MapPin, Move, Check, AlertCircle, ChevronDown, ChevronUp, X } from 'lucide-react'
import { cn } from '@/lib/utils'
import type { Checkpoint } from './CheckpointMap'
import { VectorMap } from './Map'

interface CheckpointPlacementAdminProps {
    checkpoints: Checkpoint[]
    onUpdateCheckpoints: (updated: Checkpoint[]) => void
}

const CATEGORY_COLORS: Record<string, string> = {
    academic: 'bg-blue-400 text-black',
    party: 'bg-pink-400 text-black',
    sports: 'bg-emerald-400 text-black',
    start: 'bg-amber-400 text-black',
    afterparty: 'bg-purple-400 text-black',
    default: 'bg-blush-pop-400 text-black',
}

export function CheckpointPlacementAdmin({
    checkpoints,
    onUpdateCheckpoints,
}: CheckpointPlacementAdminProps) {
    const [selectedId, setSelectedId] = React.useState<string | null>(null)
    const [placingId, setPlacingId] = React.useState<string | null>(null)

    const selectedCheckpoint = checkpoints.find((cp) => cp.id === selectedId)
    const placingCheckpoint = checkpoints.find((cp) => cp.id === placingId)

    const handleMapClick = (e: MapMouseEvent) => {
        if (!placingId) return

        const { lng, lat } = e.lngLat
        const updated = checkpoints.map((cp) =>
            cp.id === placingId ? { ...cp, longitude: lng, latitude: lat } : cp
        )

        onUpdateCheckpoints(updated)
        setSelectedId(placingId)
        setPlacingId(null)
    }

    const handleDragEnd = (id: string, lng: number, lat: number) => {
        const updated = checkpoints.map((cp) =>
            cp.id === id ? { ...cp, longitude: lng, latitude: lat } : cp
        )
        onUpdateCheckpoints(updated)
    }

    return (
        /* FIXED: Added 'isolate' to ensure MapLibre canvas stack honors outer rounded overflow clipping */
        <div className="flex h-full min-h-0 flex-1 w-full overflow-hidden rounded-xl border-2 border-black bg-white shadow-xl isolate">
            {/* Sidebar List */}
            <div className="flex w-96 lg:w-[420px] shrink-0 flex-col border-r-2 border-black bg-white min-h-0">
                <div className="border-b-2 border-black p-4 shrink-0 bg-white">
                    <h3 className="font-extrabold text-base uppercase tracking-tight text-black">
                        Checkpoint Locations
                    </h3>
                    <p className="mt-0.5 text-xs font-medium text-black/70">
                        Select a checkpoint to expand details, or click the map to set/move its position.
                    </p>
                </div>

                {/* List items with inset dividers */}
                <div className="flex-1 overflow-y-auto min-h-0 divide-y-0">
                    {checkpoints.map((cp, idx) => {
                        const hasLocation = cp.latitude !== 0 && cp.longitude !== 0
                        const isSelected = selectedId === cp.id
                        const isPlacing = placingId === cp.id

                        const categoryKey = cp.category && CATEGORY_COLORS[cp.category]
                            ? cp.category
                            : 'default'
                        const badgeColorClass = CATEGORY_COLORS[categoryKey]

                        return (
                            <React.Fragment key={cp.id}>
                                <div
                                    onClick={() => {
                                        setSelectedId(isSelected ? null : cp.id)
                                        if (!hasLocation && !isSelected) setPlacingId(cp.id)
                                    }}
                                    className={cn(
                                        'flex flex-col gap-2.5 p-4 cursor-pointer transition-colors bg-white',
                                        isPlacing
                                            ? 'bg-amber-100'
                                            : isSelected
                                                ? 'bg-blush-pop-50/70'
                                                : 'hover:bg-black/5'
                                    )}
                                >
                                    {/* Header Row */}
                                    <div className="flex items-start justify-between gap-2">
                                        <div className="flex items-center gap-2.5 min-w-0">
                                            <span
                                                className={cn(
                                                    'flex h-7 w-7 shrink-0 items-center justify-center rounded-full border border-black text-xs font-bold shadow-2xs [&>svg]:h-3.5 [&>svg]:w-3.5',
                                                    badgeColorClass
                                                )}
                                            >
                                                {cp.icon ?? cp.number ?? '•'}
                                            </span>
                                            <div className="flex flex-col min-w-0">
                                                <span className="font-bold text-sm text-black leading-tight truncate">
                                                    {cp.name}
                                                </span>
                                                {cp.category && (
                                                    <span className="mt-0.5 text-[10px] uppercase tracking-wider font-extrabold text-black/60">
                                                        {cp.category}
                                                    </span>
                                                )}
                                            </div>
                                        </div>

                                        <div className="flex items-center gap-1.5 shrink-0">
                                            {hasLocation ? (
                                                <span className="flex items-center text-[10px] font-bold text-black bg-emerald-300 px-2 py-0.5 rounded border border-black">
                                                    <Check className="mr-1 h-3 w-3 stroke-[3]" /> Placed
                                                </span>
                                            ) : (
                                                <span className="flex items-center text-[10px] font-bold text-black bg-amber-300 px-2 py-0.5 rounded border border-black">
                                                    <AlertCircle className="mr-1 h-3 w-3 stroke-[3]" /> Unset
                                                </span>
                                            )}
                                            {isSelected ? (
                                                <ChevronUp className="h-4 w-4 text-black" />
                                            ) : (
                                                <ChevronDown className="h-4 w-4 text-black/40" />
                                            )}
                                        </div>
                                    </div>

                                    {/* Expanded Details Body */}
                                    {isSelected && (
                                        <div className="flex flex-col gap-2 pt-1.5 border-t border-black/10 text-xs">
                                            {cp.description ? (
                                                <p className="text-black/80 font-medium leading-relaxed whitespace-pre-line">
                                                    {cp.description}
                                                </p>
                                            ) : (
                                                <p className="text-black/40 italic text-[11px]">No description provided.</p>
                                            )}

                                            {hasLocation && (
                                                <div className="flex items-center gap-3 text-[11px] font-mono font-bold text-black/70 bg-black/5 p-1.5 rounded border border-black/10">
                                                    <span>Lat: {cp.latitude.toFixed(5)}</span>
                                                    <span>Lng: {cp.longitude.toFixed(5)}</span>
                                                </div>
                                            )}
                                        </div>
                                    )}

                                    {/* Actions Footer - Squared, high-contrast button */}
                                    <div className="flex items-center gap-2 pt-1">
                                        <button
                                            type="button"
                                            onClick={(e) => {
                                                e.stopPropagation()
                                                setSelectedId(cp.id)
                                                setPlacingId(isPlacing ? null : cp.id)
                                            }}
                                            className={cn(
                                                'flex-1 flex items-center justify-center gap-1.5 rounded-md py-1.5 text-xs font-extrabold border-2 border-black transition-colors shadow-2xs',
                                                isPlacing
                                                    ? 'bg-amber-400 text-black'
                                                    : 'bg-white text-black hover:bg-blush-pop-100'
                                            )}
                                        >
                                            <MapPin className="h-3.5 w-3.5" />
                                            {isPlacing ? 'Click Map to Place...' : hasLocation ? 'Reposition' : 'Set Position'}
                                        </button>
                                    </div>
                                </div>

                                {/* Inset dividing border */}
                                {idx < checkpoints.length - 1 && (
                                    <div className="mx-4 border-b border-black/20" />
                                )}
                            </React.Fragment>
                        )
                    })}
                </div>
            </div>

            {/* Map View Container - FIXED: Added overflow-hidden to prevent canvas overflow at corners */}
            <div className="relative flex-1 min-h-0 overflow-hidden">
                {placingCheckpoint && (
                    <div className="absolute top-4 left-1/2 -translate-x-1/2 z-10 flex items-center gap-2 rounded-full bg-black px-4 py-2 text-xs font-bold text-white shadow-xl border-2 border-white">
                        <Move className="h-4 w-4 animate-bounce text-amber-400" />
                        Click anywhere on the map to place "{placingCheckpoint.name}"
                    </div>
                )}

                <VectorMap
                    className={cn(placingId && 'cursor-crosshair')}
                    onClick={handleMapClick}
                >
                    {checkpoints.map((cp) => {
                        if (!cp.latitude || !cp.longitude) return null
                        const isSelected = selectedId === cp.id

                        const categoryKey = cp.category && CATEGORY_COLORS[cp.category]
                            ? cp.category
                            : 'default'
                        const markerColorClass = CATEGORY_COLORS[categoryKey]

                        return (
                            <Marker
                                key={cp.id}
                                longitude={cp.longitude}
                                latitude={cp.latitude}
                                draggable
                                anchor="center"
                                onDragEnd={(e) => handleDragEnd(cp.id, e.lngLat.lng, e.lngLat.lat)}
                                onClick={(e) => {
                                    e.originalEvent.stopPropagation()
                                    setSelectedId(cp.id)
                                }}
                            >
                                <div className="relative group flex flex-col items-center cursor-grab active:cursor-grabbing">
                                    <div
                                        className={cn(
                                            'flex h-9 w-9 items-center justify-center rounded-full border-2 border-black text-xs font-bold shadow-md transition-transform hover:scale-110 [&>svg]:h-4 [&>svg]:w-4',
                                            markerColorClass,
                                            isSelected && 'ring-4 ring-black/40 scale-110'
                                        )}
                                        style={cp.color ? { backgroundColor: cp.color } : undefined}
                                    >
                                        {cp.icon ?? cp.number ?? '•'}
                                    </div>

                                    <span className="absolute top-full mt-1 left-1/2 -translate-x-1/2 pointer-events-none whitespace-nowrap bg-white px-2 py-0.5 text-xs font-bold text-black shadow-sm border-2 border-black">
                                        {cp.name}
                                    </span>
                                </div>
                            </Marker>
                        )
                    })}

                    {selectedCheckpoint && selectedCheckpoint.latitude !== 0 && (
                        <Popup
                            longitude={selectedCheckpoint.longitude}
                            latitude={selectedCheckpoint.latitude}
                            anchor="bottom"
                            offset={18}
                            onClose={() => setSelectedId(null)}
                            closeOnClick={true}
                            focusAfterOpen={false}
                            className="[&_.maplibregl-popup-content]:p-0 [&_.maplibregl-popup-content]:rounded-md [&_.maplibregl-popup-content]:shadow-xl [&_.maplibregl-popup-content]:border-2 [&_.maplibregl-popup-content]:border-black [&_.maplibregl-popup-close-button]:hidden"
                        >
                            <div className="relative min-w-[200px] max-w-xs p-3.5 bg-white">
                                <button
                                    type="button"
                                    onClick={() => setSelectedId(null)}
                                    className="absolute top-2.5 right-2.5 flex h-5 w-5 items-center justify-center rounded-full text-black/50 hover:bg-blush-pop-100 hover:text-black transition-colors"
                                >
                                    <X className="h-3.5 w-3.5" />
                                </button>

                                <div className="flex items-center gap-2.5 pr-6">
                                    <span
                                        className={cn(
                                            'flex h-7 w-7 shrink-0 items-center justify-center rounded-full border border-black text-xs font-bold shadow-2xs [&>svg]:h-3.5 [&>svg]:w-3.5',
                                            CATEGORY_COLORS[
                                            selectedCheckpoint.category && CATEGORY_COLORS[selectedCheckpoint.category]
                                                ? selectedCheckpoint.category
                                                : 'default'
                                            ]
                                        )}
                                    >
                                        {selectedCheckpoint.icon ?? selectedCheckpoint.number ?? '•'}
                                    </span>
                                    <h4 className="font-extrabold text-sm text-black leading-tight">
                                        {selectedCheckpoint.name}
                                    </h4>
                                </div>

                                {selectedCheckpoint.description && (
                                    <p className="mt-2 text-xs font-medium text-black/80 leading-relaxed border-t border-black/10 pt-2">
                                        {selectedCheckpoint.description}
                                    </p>
                                )}

                                <div className="mt-2 flex items-center gap-3 text-[10px] font-mono font-bold text-black/70 bg-black/5 p-1.5 rounded border border-black/10">
                                    <span>Lat: {selectedCheckpoint.latitude.toFixed(5)}</span>
                                    <span>Lng: {selectedCheckpoint.longitude.toFixed(5)}</span>
                                </div>
                            </div>
                        </Popup>
                    )}
                </VectorMap>
            </div>
        </div>
    )
}
