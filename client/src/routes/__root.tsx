import * as React from 'react'
import type { QueryClient } from '@tanstack/react-query'
import { Outlet, createRootRouteWithContext } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'

import { cn } from '@/lib/utils'

export interface RouterContext {
    queryClient: QueryClient
}

export const Route = createRootRouteWithContext<RouterContext>()({
    component: RootComponent,
})

function RootComponent() {
    return (
        <React.Fragment>
            <header>
                <nav className={cn('p-2')}>
                    <h1 className={cn('text-sm md:text-base font-extrabold tracking-tighter uppercase leading-none text-black flex items-center gap-3')}>
                        <span>Fuksiseikkailu</span>
                        <span className="hidden sm:inline text-black/80">Gulisäventyret</span>
                        <span className="hidden md:inline text-black/80">Fresher Adventure</span>
                    </h1>
                </nav>
            </header>
            <main className={cn('flex-1 p-2 flex flex-col min-h-0 overflow-hidden pb-4')}>
                <Outlet />
            </main>
            <footer className={cn('p-2')}>
                footer
            </footer>
            <TanStackRouterDevtools />
            <ReactQueryDevtools />
        </React.Fragment>
    )
}
