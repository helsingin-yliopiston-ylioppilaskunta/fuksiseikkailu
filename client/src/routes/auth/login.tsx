import { LoginForm } from '@/components/LoginForm'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/auth/login')({
    component: RouteComponent,
})

function RouteComponent() {
    return (
        <div>
            <LoginForm />
        </div>
    )
}
