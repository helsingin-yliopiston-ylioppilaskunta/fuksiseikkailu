import * as React from 'react'
import { useForm } from '@tanstack/react-form'
import { useMutation } from '@tanstack/react-query'
import { KeyRound, Mail, ArrowRight, CheckCircle2, LogOut } from 'lucide-react'

import {
    requestOtpMutation,
    verifyOtpMutation,
} from '@/api/generated/@tanstack/react-query.gen'
import { useAuth } from '@/auth/AuthContext'

export function LoginForm() {
    const { isAuthenticated, login, logout } = useAuth()
    const [step, setStep] = React.useState<'request' | 'verify'>('request')
    const [sentEmail, setSentEmail] = React.useState('')

    // Mutations with custom toast meta
    const requestOtp = useMutation({
        ...requestOtpMutation(),
        mutationKey: ['auth', 'requestOtp'],
        meta: {
            loadingMessage: 'Sending magic code...',
            successMessage: 'OTP code sent to your email!',
        },
    })

    const verifyOtp = useMutation({
        ...verifyOtpMutation(),
        mutationKey: ['auth', 'verifyOtp'],
        meta: {
            loadingMessage: 'Verifying code...',
            successMessage: 'Authenticated successfully!',
        },
    })

    // Step 1 Form: Email Request
    const requestForm = useForm({
        defaultValues: { email: '' },
        onSubmit: async ({ value }) => {
            const email = value.email.trim().toLowerCase()
            await requestOtp.mutateAsync({ body: { email } })
            setSentEmail(email)
            setStep('verify')
        },
    })

    // Step 2 Form: Code Verification
    const verifyForm = useForm({
        defaultValues: { code: '' },
        onSubmit: async ({ value }) => {
            const res = await verifyOtp.mutateAsync({
                body: {
                    email: sentEmail,
                    code: value.code.trim(),
                },
            })

            // Guard: Ensure token actually exists on the response payload
            if (!res?.access_token) {
                throw new Error('Authentication succeeded but no access token was returned by the server.')
            }

            // Store token in localStorage and update auth state
            login(res.access_token)
        },
    })

    if (isAuthenticated) {
        return (
            <div className="flex w-full max-w-sm flex-col overflow-hidden rounded-xl border-2 border-black bg-white shadow-xl isolate">
                <div className="flex items-center gap-3 border-b-2 border-black bg-emerald-300 p-4">
                    <CheckCircle2 className="h-6 w-6 stroke-[2.5] text-black" />
                    <div>
                        <h3 className="text-sm font-black uppercase tracking-wide text-black">
                            Authenticated
                        </h3>
                        <p className="text-[11px] font-bold text-black/80">
                            Your session token is saved in local storage.
                        </p>
                    </div>
                </div>

                <div className="p-4">
                    <button
                        type="button"
                        onClick={logout}
                        className="flex w-full items-center justify-center gap-2 rounded-md border-2 border-black bg-rose-500 py-2 text-xs font-extrabold text-white shadow-2xs transition-colors hover:bg-rose-600"
                    >
                        <LogOut className="h-4 w-4" />
                        Log Out / Clear Token
                    </button>
                </div>
            </div>
        )
    }

    return (
        <div className="flex w-full max-w-sm flex-col overflow-hidden rounded-xl border-2 border-black bg-white shadow-xl isolate">
            {/* Header */}
            <div className="border-b-2 border-black bg-amber-400 p-4 text-black">
                <h3 className="text-base font-black uppercase tracking-tight">
                    {step === 'request' ? 'Event Login' : 'Enter Verification Code'}
                </h3>
                <p className="mt-0.5 text-xs font-bold text-black/80">
                    {step === 'request'
                        ? 'Enter your organizer email to receive a login code.'
                        : `Code sent to ${sentEmail}`}
                </p>
            </div>

            {/* Step 1: Request OTP */}
            {step === 'request' ? (
                <form
                    onSubmit={(e) => {
                        e.preventDefault()
                        e.stopPropagation()
                        requestForm.handleSubmit()
                    }}
                    className="flex flex-col gap-4 p-4 text-xs font-bold text-black"
                >
                    <requestForm.Field
                        name="email"
                        validators={{
                            onChange: ({ value }) =>
                                !value.includes('@') ? 'Enter a valid email' : undefined,
                        }}
                    >
                        {(field) => (
                            <div className="flex flex-col gap-1">
                                <label htmlFor={field.name} className="uppercase tracking-wider text-[10px] text-black/70">
                                    Email Address
                                </label>
                                <div className="relative">
                                    <Mail className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-black/50" />
                                    <input
                                        id={field.name}
                                        type="email"
                                        value={field.state.value}
                                        onChange={(e) => field.handleChange(e.target.value)}
                                        placeholder="organizer@helsinki.fi"
                                        className="w-full rounded-md border-2 border-black bg-white py-2 pl-9 pr-3 text-xs font-bold text-black placeholder:text-black/30 focus:outline-none focus:ring-2 focus:ring-black/20"
                                    />
                                </div>
                            </div>
                        )}
                    </requestForm.Field>

                    <button
                        type="submit"
                        disabled={requestOtp.isPending}
                        className="flex items-center justify-center gap-2 rounded-md border-2 border-black bg-amber-400 py-2.5 text-xs font-extrabold text-black shadow-2xs hover:bg-amber-300 transition-colors disabled:opacity-50"
                    >
                        Send Login Code <ArrowRight className="h-4 w-4" />
                    </button>
                </form>
            ) : (
                /* Step 2: Verify OTP */
                <form
                    onSubmit={(e) => {
                        e.preventDefault()
                        e.stopPropagation()
                        verifyForm.handleSubmit()
                    }}
                    className="flex flex-col gap-4 p-4 text-xs font-bold text-black"
                >
                    <verifyForm.Field name="code">
                        {(field) => (
                            <div className="flex flex-col gap-1">
                                <label htmlFor={field.name} className="uppercase tracking-wider text-[10px] text-black/70">
                                    6-Digit Verification Code
                                </label>
                                <div className="relative">
                                    <KeyRound className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-black/50" />
                                    <input
                                        id={field.name}
                                        type="text"
                                        maxLength={6}
                                        value={field.state.value}
                                        onChange={(e) => field.handleChange(e.target.value)}
                                        placeholder="123456"
                                        className="w-full rounded-md border-2 border-black bg-white py-2 pl-9 pr-3 font-mono text-sm tracking-widest font-black text-black placeholder:text-black/30 focus:outline-none focus:ring-2 focus:ring-black/20"
                                    />
                                </div>
                            </div>
                        )}
                    </verifyForm.Field>

                    <div className="flex gap-2">
                        <button
                            type="button"
                            onClick={() => setStep('request')}
                            className="flex-1 rounded-md border-2 border-black bg-white py-2 text-xs font-extrabold text-black hover:bg-black/5"
                        >
                            Back
                        </button>
                        <button
                            type="submit"
                            disabled={verifyOtp.isPending}
                            className="flex-2 rounded-md border-2 border-black bg-emerald-400 py-2 text-xs font-extrabold text-black hover:bg-emerald-300 transition-colors disabled:opacity-50"
                        >
                            Verify & Log In
                        </button>
                    </div>
                </form>
            )}
        </div>
    )
}
