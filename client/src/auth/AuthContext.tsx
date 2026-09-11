import * as React from 'react'

interface AuthContextType {
    token: string | null
    isAuthenticated: boolean
    login: (token: string) => void
    logout: () => void
}

const AuthContext = React.createContext<AuthContextType | undefined>(undefined)

const TOKEN_KEY = 'access_token'

export function AuthProvider({ children }: { children: React.ReactNode }) {
    const [token, setToken] = React.useState<string | null>(() => {
        return localStorage.getItem(TOKEN_KEY)
    })

    const login = React.useCallback((newToken: string) => {
        localStorage.setItem(TOKEN_KEY, newToken)
        setToken(newToken)
    }, [])

    const logout = React.useCallback(() => {
        localStorage.removeItem(TOKEN_KEY)
        setToken(null)
    }, [])

    const value = React.useMemo(
        () => ({
            token,
            isAuthenticated: Boolean(token),
            login,
            logout,
        }),
        [token, login, logout]
    )

    return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
}

export function useAuth() {
    const context = React.useContext(AuthContext)
    if (!context) {
        throw new Error('useAuth must be used within an AuthProvider')
    }
    return context
}
