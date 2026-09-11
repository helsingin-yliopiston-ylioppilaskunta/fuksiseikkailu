/**
 * Safely extracts human-readable error messages from Hey-API / TanStack Query error objects.
 */
export function getApiErrorMessage(error: unknown): string {
    if (!error) return ''

    if (typeof error === 'object' && error !== null) {
        const err = error as Record<string, any>

        // 1. Extract HTTP status code from Hey-API response
        const status = err.status ?? err.statusCode ?? err.response?.status

        if (status === 401) {
            return '401 Unauthorized: You must be logged in as an Admin to create or edit checkpoints.'
        }

        if (status === 403) {
            return '403 Forbidden: You do not have permission to perform this action.'
        }

        // 2. Hey-API stores the parsed response JSON body under `err.error`
        const body = err.error ?? err.body

        if (body) {
            if (typeof body === 'string') return body
            if (typeof body === 'object') {
                if (body.message) return String(body.message)
                if (body.error) return String(body.error)
            }
        }

        if (err.message && typeof err.message === 'string') {
            return err.message
        }
    }

    return 'An unexpected server error occurred. Check inputs or network logs.'
}
