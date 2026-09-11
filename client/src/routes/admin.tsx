import * as React from 'react'
import { createFileRoute } from '@tanstack/react-router'
import { CheckpointPlacementAdmin } from '@/components/CheckpointAdmin'
import type { Checkpoint } from '@/components/CheckpointMap'
import { Flag, PartyPopper, Beer, Sparkles, Terminal } from 'lucide-react'

export const Route = createFileRoute('/admin')({
    component: RouteComponent,
})

// Initial mock checkpoints for Fuksiseikkailu
const INITIAL_CHECKPOINTS: Checkpoint[] = [
    {
        id: 'start-senaatintori',
        name: 'Starting Area (Senaatintori)',
        description: 'Registration, event material collection, and official kickoff.',
        latitude: 60.1695,
        longitude: 24.9525,
        icon: <Flag />,
        category: 'start',
    },
    {
        id: 'cp-1-suomenlinna',
        number: 1,
        name: 'Checkpoint 1: Suomenlinna Ferry',
        description: 'Maritime orientation challenge.',
        latitude: 60.1675,
        longitude: 24.9538,
        category: 'academic',
    },
    {
        id: 'cp-2-päärakennus',
        number: 2,
        name: 'Checkpoint 2: Main Building',
        description: 'University trivia and speed puzzle.',
        latitude: 60.1699,
        longitude: 24.9484,
        category: 'academic',
    },
    {
        id: 'cp-3-kaisaniemi',
        number: 3,
        name: 'Checkpoint 3: Kaisaniemi Park',
        description: 'Outdoor team sports relay.',
        latitude: 60.1742,
        longitude: 24.9462,
        category: 'sports',
    },
    {
        id: 'cp-4-kamppi',
        number: 4,
        name: 'Checkpoint 4: Narinkkatori',
        description: 'Student association dance challenge.',
        latitude: 60.169,
        longitude: 24.936,
        category: 'party',
    },
    {
        id: 'cp-5-kumpula',
        number: 5,
        name: 'Checkpoint 5: Kumpula Campus',
        description: 'Unset location example (needs placement).',
        latitude: 0, // Unset coordinate
        longitude: 0,
        icon: <Sparkles />,
        category: 'academic',
    },
    {
        id: 'cp-6-alppipuisto',
        number: 6,
        name: 'Checkpoint 6: Alppipuisto Park',
        description: 'Unset location example (needs placement).',
        latitude: 0, // Unset coordinate
        longitude: 0,
        icon: <Beer />,
        category: 'sports',
    },
    {
        id: 'afterparty-tavastia',
        name: 'Official Afterparty (Tavastia)',
        description: 'Nighttime celebration and winner announcement.',
        latitude: 60.169,
        longitude: 24.933,
        icon: <PartyPopper />,
        category: 'afterparty',
    },
]

function RouteComponent() {
    const [checkpoints, setCheckpoints] = React.useState<Checkpoint[]>(INITIAL_CHECKPOINTS)

    return (
        <div className="flex flex-1 flex-col min-h-0 h-full w-full max-w-7xl mx-auto gap-3">
            <header className="flex items-center justify-between shrink-0">
                <div>
                    <h1 className="text-xl font-extrabold uppercase tracking-tight text-black">
                        Checkpoint Admin Tool
                    </h1>
                    <p className="text-xs font-bold text-black/70">
                        Drag markers on the map or click "Set Position" to locate unset checkpoints.
                    </p>
                </div>
                <button
                    type="button"
                    onClick={() => console.log('Current Checkpoints:', checkpoints)}
                    className="flex items-center gap-1.5 rounded-md border-2 border-black bg-white px-3 py-1.5 text-xs font-extrabold text-black shadow-2xs hover:bg-blush-pop-100 transition-colors cursor-pointer"
                >
                    <Terminal className="h-3.5 w-3.5" />
                    Log Active State
                </button>
            </header>

            <CheckpointPlacementAdmin
                checkpoints={checkpoints}
                onUpdateCheckpoints={setCheckpoints}
            />
        </div>
    )
}
