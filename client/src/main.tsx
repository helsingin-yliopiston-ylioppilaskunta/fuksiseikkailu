import React from 'react'
import ReactDOM from 'react-dom/client'
import {
    MutationCache,
    QueryClient,
    QueryClientProvider,
} from '@tanstack/react-query'
import { RouterProvider, createRouter } from '@tanstack/react-router'
import { Toaster, toast } from 'react-hot-toast'

import './index.css'
import './api/client'
import { routeTree } from './routeTree.gen'
import { getApiErrorMessage } from './lib/errors'
import { AuthProvider } from './auth/AuthContext'

// Module augmentation for custom mutation toast metadata
declare module '@tanstack/react-query' {
    interface Register {
        mutationMeta: {
            loadingMessage?: string
            successMessage?: string | ((data: unknown) => string)
            errorMessage?: string | ((error: unknown) => string)
            /** Set to true to bypass automatic toast notifications */
            silent?: boolean
        }
    }
}

// Map to associate active toast IDs with mutation IDs safely
const activeToasts = new Map<number, string>()

// QueryClient with global MutationCache toast handlers
const queryClient = new QueryClient({
    mutationCache: new MutationCache({
        onMutate: (_variables, mutation) => {
            if (mutation.meta?.silent) return

            const loadingMsg =
                mutation.meta?.loadingMessage || 'Processing request...'
            const toastId = toast.loading(loadingMsg)
            activeToasts.set(mutation.mutationId, toastId)
        },
        onSuccess: (data, _variables, _context, mutation) => {
            if (mutation.meta?.silent) return

            const toastId = activeToasts.get(mutation.mutationId)
            const successMsg =
                typeof mutation.meta?.successMessage === 'function'
                    ? mutation.meta.successMessage(data)
                    : mutation.meta?.successMessage || 'Operation completed successfully!'

            if (toastId) {
                toast.success(successMsg, { id: toastId })
                activeToasts.delete(mutation.mutationId)
            } else {
                toast.success(successMsg)
            }
        },
        onError: (error, _variables, _context, mutation) => {
            if (mutation.meta?.silent) return

            const toastId = activeToasts.get(mutation.mutationId)
            const defaultErr = getApiErrorMessage(error)
            const errorMsg =
                typeof mutation.meta?.errorMessage === 'function'
                    ? mutation.meta.errorMessage(error)
                    : mutation.meta?.errorMessage || defaultErr

            if (toastId) {
                toast.error(errorMsg, { id: toastId })
                activeToasts.delete(mutation.mutationId)
            } else {
                toast.error(errorMsg)
            }
        },
    }),
})

const router = createRouter({
    routeTree,
    context: {
        queryClient,
    },
})

declare module '@tanstack/react-router' {
    interface Register {
        router: typeof router
    }
}

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <AuthProvider>
            <QueryClientProvider client={queryClient}>
                <RouterProvider router={router} />
                <Toaster
                    position="bottom-right"
                    toastOptions={{
                        className:
                            '!border-2 !border-black !rounded-md !shadow-xl !font-bold !text-xs !text-black',
                        duration: 4000,
                        style: {
                            background: '#ffffff',
                            color: '#000000',
                        },
                    }}
                />
            </QueryClientProvider>
        </AuthProvider>
    </React.StrictMode>,
)
