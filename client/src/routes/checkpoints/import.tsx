import { CheckpointBatchImport } from '@/components/CheckpointBatchImport'
import { cn } from '@/lib/utils'
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { ArrowLeft } from 'lucide-react'

export const Route = createFileRoute('/checkpoints/import')({
    component: RouteComponent,
})

function RouteComponent() {
    const navigate = useNavigate()

    const handleNavigateBack = () => {
        navigate({ to: '/checkpoints' })
    }

    return (
        <div className={cn('flex flex-col gap-4 p-4')}>
            {/* Navigation Header */}
            <div>
                <button
                    type="button"
                    onClick={handleNavigateBack}
                    className="flex items-center gap-1.5 rounded-md border-2 border-black bg-white px-3 py-1.5 text-xs font-extrabold text-black shadow-2xs hover:bg-black/5 transition-colors"
                >
                    <ArrowLeft className="h-4 w-4" />
                    Back to Checkpoints
                </button>
            </div>

            {/* Batch Import Container */}
            <CheckpointBatchImport
                onSuccess={handleNavigateBack}
                onCancel={handleNavigateBack}
            />
        </div>
    )
}
