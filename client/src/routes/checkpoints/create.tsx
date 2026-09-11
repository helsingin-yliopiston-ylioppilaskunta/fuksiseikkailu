import { CheckpointForm } from '@/components/CheckpointForm'
import { cn } from '@/lib/utils'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/checkpoints/create')({
    component: RouteComponent,
})

function RouteComponent() {
    return (
        <div className={cn('pt-2')}>
            <CheckpointForm />
        </div>
    )
}
