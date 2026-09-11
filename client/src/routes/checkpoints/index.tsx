import { CheckpointList } from '@/components/CheckpointList'
import { cn } from '@/lib/utils'
import { createFileRoute, Link } from '@tanstack/react-router'
import { FileSpreadsheet, Plus } from 'lucide-react'

export const Route = createFileRoute('/checkpoints/')({
    component: RouteComponent,
})

function RouteComponent() {
    return (
        <div className={cn('flex h-full w-full flex-col items-center gap-4 overflow-y-auto p-4')}>
            {/* Top Action Header Bar */}
            <div className="flex w-full max-w-2xl items-center justify-between gap-3">
                <div>
                    <h2 className="text-xl font-black uppercase tracking-tight text-black">
                        Checkpoints Management
                    </h2>
                    <p className="text-xs font-bold text-black/70">
                        Create, edit, or import event checkpoints.
                    </p>
                </div>

                <div className="flex items-center gap-2">
                    {/* Import Checkpoints Link */}
                    <Link
                        to="/checkpoints/import"
                        className="flex items-center gap-1.5 rounded-md border-2 border-black bg-white px-3 py-1.5 text-xs font-extrabold text-black shadow-2xs hover:bg-black/5 transition-colors"
                    >
                        <FileSpreadsheet className="h-4 w-4" />
                        <span className="hidden sm:inline">Batch Import</span>
                    </Link>

                    {/* Create Checkpoint Link */}
                    <Link
                        to="/checkpoints/create"
                        className="flex items-center gap-1.5 rounded-md border-2 border-black bg-amber-400 px-3 py-1.5 text-xs font-black text-black shadow-2xs hover:bg-amber-300 transition-colors"
                    >
                        <Plus className="h-4 w-4 stroke-[3]" />
                        <span>New Checkpoint</span>
                    </Link>
                </div>
            </div>

            {/* Main Filterable Checkpoint List */}
            <CheckpointList />
        </div>
    )
}
