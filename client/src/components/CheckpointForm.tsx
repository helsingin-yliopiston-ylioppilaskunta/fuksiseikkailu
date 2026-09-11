import { useForm } from '@tanstack/react-form'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { AlertCircle, Loader2, Save, X } from 'lucide-react'

import {
    createCheckpointMutation,
    updateCheckpointMutation,
} from '@/api/generated/@tanstack/react-query.gen'
import type { Checkpoint, CheckpointCategory, CreateCheckpoint, UpdateCheckpoint } from '@/api/generated/types.gen'
import { cn } from '@/lib/utils'
import { getApiErrorMessage } from '@/lib/errors'

interface CheckpointFormProps {
    /** If provided, form operates in "Update" mode; otherwise "Create" mode */
    initialData?: Checkpoint | null
    /** Callback fired after successful submission */
    onSuccess?: () => void
    /** Callback fired when user cancels form editing */
    onCancel?: () => void
}

const CATEGORY_OPTIONS: { label: string; value: CheckpointCategory }[] = [
    { label: 'Subject / Ainejärjestö', value: 'subject' },
    { label: 'Nation / Osakunta', value: 'nation' },
    { label: 'Hobby / Kerho', value: 'hobby' },
    { label: 'HYY', value: 'hyy' },
    { label: 'Yliopisto', value: 'yliopisto' },
    { label: 'Other / Muu', value: 'other' },
]

export function CheckpointForm({ initialData, onSuccess, onCancel }: CheckpointFormProps) {
    const queryClient = useQueryClient()
    const isEditing = Boolean(initialData)

    // 1. TanStack Query Mutations with explicit mutationKey
    const createMutation = useMutation({
        ...createCheckpointMutation(),
        mutationKey: ['checkpoints', 'create'],
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['checkpoints'] })
            onSuccess?.()
        },
    })

    const updateMutation = useMutation({
        ...updateCheckpointMutation(),
        mutationKey: ['checkpoints', 'update', initialData?.id],
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['checkpoints'] })
            onSuccess?.()
        },
    })

    const isSubmitting = createMutation.isPending || updateMutation.isPending
    const activeError = createMutation.error || updateMutation.error
    const errorMessage = getApiErrorMessage(activeError)

    // 2. TanStack Form Hook
    const form = useForm({
        defaultValues: {
            name: initialData?.name ?? '',
            number: initialData?.number ?? ('' as number | string),
            category: (initialData?.category ?? 'subject') as CheckpointCategory,
            location_name: initialData?.location_name ?? '',
            latitude: initialData?.latitude ?? ('' as number | string),
            longitude: initialData?.longitude ?? ('' as number | string),
            lanes: initialData?.lanes ?? 1,
            accessible: initialData?.accessible ?? true,
            checkpoint_description: typeof initialData?.checkpoint_description === 'string'
                ? initialData.checkpoint_description
                : initialData?.checkpoint_description
                    ? JSON.stringify(initialData.checkpoint_description)
                    : '',
            contact_person: initialData?.contact_person ?? '',
            contact_email: initialData?.contact_email ?? '',
            contact_phone: initialData?.contact_phone ?? '',
            url: initialData?.url ?? '',
            cancelled: initialData?.cancelled ?? false,
        },
        onSubmit: async ({ value }) => {
            const payload: CreateCheckpoint | UpdateCheckpoint = {
                name: value.name.trim(),
                number: value.number !== '' ? Number(value.number) : null,
                category: value.category,
                location_name: value.location_name.trim() || null,
                latitude: value.latitude !== '' ? Number(value.latitude) : 0,
                longitude: value.longitude !== '' ? Number(value.longitude) : 0,
                lanes: Number(value.lanes) || 1,
                accessible: value.accessible,
                checkpoint_description: value.checkpoint_description.trim() || null,
                contact_person: value.contact_person.trim() || null,
                contact_email: value.contact_email.trim() || null,
                contact_phone: value.contact_phone.trim() || null,
                url: value.url.trim() || null,
                cancelled: value.cancelled,
            }

            if (isEditing && initialData) {
                await updateMutation.mutateAsync({
                    path: { id: initialData.id },
                    body: payload as UpdateCheckpoint,
                })
            } else {
                await createMutation.mutateAsync({
                    body: payload as CreateCheckpoint,
                })
            }
        },
    })

    return (
        <div className="flex w-full max-w-xl flex-col overflow-hidden rounded-xl border-2 border-black bg-white shadow-xl isolate">
            {/* Form Header */}
            <div className="flex items-center justify-between border-b-2 border-black p-4 bg-white shrink-0">
                <div>
                    <h3 className="text-base font-extrabold uppercase tracking-tight text-black">
                        {isEditing ? `Edit Checkpoint: ${initialData?.name}` : 'Create New Checkpoint'}
                    </h3>
                    <p className="mt-0.5 text-xs font-medium text-black/70">
                        {isEditing
                            ? 'Update operational info, contact details, or location coordinates.'
                            : 'Add a new checkpoint to the event database.'}
                    </p>
                </div>

                {onCancel && (
                    <button
                        type="button"
                        onClick={onCancel}
                        className="flex h-7 w-7 items-center justify-center rounded-md border-2 border-black bg-white hover:bg-black/10 transition-colors"
                    >
                        <X className="h-4 w-4 text-black" />
                    </button>
                )}
            </div>

            {/* Error Alert */}
            {activeError && (
                <div className="m-4 flex items-start gap-3 rounded-md border-2 border-black bg-rose-100 p-3 text-xs font-bold text-black shadow-2xs">
                    <AlertCircle className="h-5 w-5 shrink-0 text-rose-600 stroke-[2.5]" />
                    <div className="flex-1 min-w-0">
                        <p className="font-black uppercase tracking-wide text-rose-950">
                            Submission Failed
                        </p>
                        <p className="mt-0.5 font-medium leading-relaxed text-rose-900">
                            {errorMessage}
                        </p>
                    </div>
                </div>
            )}

            {/* Form Body */}
            <form
                onSubmit={(e) => {
                    e.preventDefault()
                    e.stopPropagation()
                    form.handleSubmit()
                }}
                className="flex flex-col gap-4 p-4 text-xs font-bold text-black"
            >
                {/* Row 1: Name & Number */}
                <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
                    <form.Field
                        name="name"
                        validators={{
                            onChange: ({ value }) => (!value.trim() ? 'Name is required' : undefined),
                        }}
                    >
                        {(field) => (
                            <div className="sm:col-span-2 flex flex-col gap-1">
                                <label htmlFor={field.name} className="uppercase tracking-wider text-[10px] text-black/70">
                                    Checkpoint Name *
                                </label>
                                <input
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    onBlur={field.handleBlur}
                                    onChange={(e) => field.handleChange(e.target.value)}
                                    placeholder="e.g. OIH Räp-Räp"
                                    className={cn(
                                        'rounded-md border-2 border-black bg-white px-3 py-1.5 text-xs font-bold text-black placeholder:text-black/30 focus:outline-none focus:ring-2 focus:ring-black/20',
                                        field.state.meta.errors.length > 0 && 'border-rose-600 bg-rose-50'
                                    )}
                                />
                                {field.state.meta.errors[0] && (
                                    <span className="text-[10px] text-rose-600 font-extrabold">
                                        {field.state.meta.errors[0]}
                                    </span>
                                )}
                            </div>
                        )}
                    </form.Field>

                    <form.Field name="number">
                        {(field) => (
                            <div className="flex flex-col gap-1">
                                <label htmlFor={field.name} className="uppercase tracking-wider text-[10px] text-black/70">
                                    Number (#)
                                </label>
                                <input
                                    id={field.name}
                                    name={field.name}
                                    type="number"
                                    value={field.state.value}
                                    onBlur={field.handleBlur}
                                    onChange={(e) => field.handleChange(e.target.value)}
                                    placeholder="e.g. 12"
                                    className="rounded-md border-2 border-black bg-white px-3 py-1.5 text-xs font-bold text-black placeholder:text-black/30 focus:outline-none focus:ring-2 focus:ring-black/20"
                                />
                            </div>
                        )}
                    </form.Field>
                </div>

                {/* Row 2: Category & Location Name */}
                <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
                    <form.Field name="category">
                        {(field) => (
                            <div className="flex flex-col gap-1">
                                <label htmlFor={field.name} className="uppercase tracking-wider text-[10px] text-black/70">
                                    Category
                                </label>
                                <select
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    onChange={(e) => field.handleChange(e.target.value as CheckpointCategory)}
                                    className="rounded-md border-2 border-black bg-white px-3 py-1.5 text-xs font-bold text-black focus:outline-none focus:ring-2 focus:ring-black/20"
                                >
                                    {CATEGORY_OPTIONS.map((opt) => (
                                        <option key={opt.value} value={opt.value}>
                                            {opt.label}
                                        </option>
                                    ))}
                                </select>
                            </div>
                        )}
                    </form.Field>

                    <form.Field name="location_name">
                        {(field) => (
                            <div className="flex flex-col gap-1">
                                <label htmlFor={field.name} className="uppercase tracking-wider text-[10px] text-black/70">
                                    Location Name
                                </label>
                                <input
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    onBlur={field.handleBlur}
                                    onChange={(e) => field.handleChange(e.target.value)}
                                    placeholder="e.g. Topelia Courtyard"
                                    className="rounded-md border-2 border-black bg-white px-3 py-1.5 text-xs font-bold text-black placeholder:text-black/30 focus:outline-none focus:ring-2 focus:ring-black/20"
                                />
                            </div>
                        )}
                    </form.Field>
                </div>

                {/* Row 3: Latitude, Longitude, Lanes */}
                <div className="grid grid-cols-3 gap-3">
                    <form.Field name="latitude">
                        {(field) => (
                            <div className="flex flex-col gap-1">
                                <label htmlFor={field.name} className="uppercase tracking-wider text-[10px] text-black/70">
                                    Latitude
                                </label>
                                <input
                                    id={field.name}
                                    name={field.name}
                                    type="number"
                                    step="any"
                                    value={field.state.value}
                                    onBlur={field.handleBlur}
                                    onChange={(e) => field.handleChange(e.target.value)}
                                    placeholder="60.1708"
                                    className="rounded-md border-2 border-black bg-white px-3 py-1.5 text-xs font-bold text-black placeholder:text-black/30 focus:outline-none focus:ring-2 focus:ring-black/20"
                                />
                            </div>
                        )}
                    </form.Field>

                    <form.Field name="longitude">
                        {(field) => (
                            <div className="flex flex-col gap-1">
                                <label htmlFor={field.name} className="uppercase tracking-wider text-[10px] text-black/70">
                                    Longitude
                                </label>
                                <input
                                    id={field.name}
                                    name={field.name}
                                    type="number"
                                    step="any"
                                    value={field.state.value}
                                    onBlur={field.handleBlur}
                                    onChange={(e) => field.handleChange(e.target.value)}
                                    placeholder="24.9502"
                                    className="rounded-md border-2 border-black bg-white px-3 py-1.5 text-xs font-bold text-black placeholder:text-black/30 focus:outline-none focus:ring-2 focus:ring-black/20"
                                />
                            </div>
                        )}
                    </form.Field>

                    <form.Field name="lanes">
                        {(field) => (
                            <div className="flex flex-col gap-1">
                                <label htmlFor={field.name} className="uppercase tracking-wider text-[10px] text-black/70">
                                    Lanes
                                </label>
                                <input
                                    id={field.name}
                                    name={field.name}
                                    type="number"
                                    min={1}
                                    value={field.state.value}
                                    onBlur={field.handleBlur}
                                    onChange={(e) => field.handleChange(Number(e.target.value))}
                                    className="rounded-md border-2 border-black bg-white px-3 py-1.5 text-xs font-bold text-black focus:outline-none focus:ring-2 focus:ring-black/20"
                                />
                            </div>
                        )}
                    </form.Field>
                </div>

                {/* Description */}
                <form.Field name="checkpoint_description">
                    {(field) => (
                        <div className="flex flex-col gap-1">
                            <label htmlFor={field.name} className="uppercase tracking-wider text-[10px] text-black/70">
                                Description & Instructions
                            </label>
                            <textarea
                                id={field.name}
                                name={field.name}
                                rows={3}
                                value={field.state.value}
                                onBlur={field.handleBlur}
                                onChange={(e) => field.handleChange(e.target.value)}
                                placeholder="Brief guidelines, requirements, or checkpoint rules..."
                                className="rounded-md border-2 border-black bg-white p-3 text-xs font-bold text-black placeholder:text-black/30 focus:outline-none focus:ring-2 focus:ring-black/20"
                            />
                        </div>
                    )}
                </form.Field>

                {/* Organizer Contacts */}
                <div className="grid grid-cols-1 sm:grid-cols-3 gap-3 pt-2 border-t border-black/10">
                    <form.Field name="contact_person">
                        {(field) => (
                            <div className="flex flex-col gap-1">
                                <label htmlFor={field.name} className="uppercase tracking-wider text-[10px] text-black/70">
                                    Contact Person
                                </label>
                                <input
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    onBlur={field.handleBlur}
                                    onChange={(e) => field.handleChange(e.target.value)}
                                    placeholder="Matti Meikäläinen"
                                    className="rounded-md border-2 border-black bg-white px-3 py-1.5 text-xs font-bold text-black placeholder:text-black/30 focus:outline-none focus:ring-2 focus:ring-black/20"
                                />
                            </div>
                        )}
                    </form.Field>

                    <form.Field name="contact_email">
                        {(field) => (
                            <div className="flex flex-col gap-1">
                                <label htmlFor={field.name} className="uppercase tracking-wider text-[10px] text-black/70">
                                    Contact Email
                                </label>
                                <input
                                    id={field.name}
                                    name={field.name}
                                    type="email"
                                    value={field.state.value}
                                    onBlur={field.handleBlur}
                                    onChange={(e) => field.handleChange(e.target.value)}
                                    placeholder="organizer@helsinki.fi"
                                    className="rounded-md border-2 border-black bg-white px-3 py-1.5 text-xs font-bold text-black placeholder:text-black/30 focus:outline-none focus:ring-2 focus:ring-black/20"
                                />
                            </div>
                        )}
                    </form.Field>

                    <form.Field name="contact_phone">
                        {(field) => (
                            <div className="flex flex-col gap-1">
                                <label htmlFor={field.name} className="uppercase tracking-wider text-[10px] text-black/70">
                                    Contact Phone
                                </label>
                                <input
                                    id={field.name}
                                    name={field.name}
                                    value={field.state.value}
                                    onBlur={field.handleBlur}
                                    onChange={(e) => field.handleChange(e.target.value)}
                                    placeholder="+358 40 1234567"
                                    className="rounded-md border-2 border-black bg-white px-3 py-1.5 text-xs font-bold text-black placeholder:text-black/30 focus:outline-none focus:ring-2 focus:ring-black/20"
                                />
                            </div>
                        )}
                    </form.Field>
                </div>

                {/* Switches: Accessible & Cancelled */}
                <div className="flex items-center justify-between gap-4 pt-2 border-t border-black/10">
                    <form.Field name="accessible">
                        {(field) => (
                            <label className="flex items-center gap-2 cursor-pointer">
                                <input
                                    type="checkbox"
                                    checked={field.state.value}
                                    onChange={(e) => field.handleChange(e.target.checked)}
                                    className="h-4 w-4 rounded border-2 border-black accent-black focus:ring-0"
                                />
                                <span className="uppercase tracking-wider text-[10px] text-black/80">
                                    Wheelchair Accessible
                                </span>
                            </label>
                        )}
                    </form.Field>

                    <form.Field name="cancelled">
                        {(field) => (
                            <label className="flex items-center gap-2 cursor-pointer">
                                <input
                                    type="checkbox"
                                    checked={field.state.value}
                                    onChange={(e) => field.handleChange(e.target.checked)}
                                    className="h-4 w-4 rounded border-2 border-black accent-rose-600 focus:ring-0"
                                />
                                <span className="uppercase tracking-wider text-[10px] text-rose-600 font-extrabold">
                                    Mark as Cancelled
                                </span>
                            </label>
                        )}
                    </form.Field>
                </div>

                {/* Submit / Cancel Buttons */}
                <div className="flex items-center justify-end gap-2 pt-3 border-t-2 border-black">
                    {onCancel && (
                        <button
                            type="button"
                            onClick={onCancel}
                            disabled={isSubmitting}
                            className="rounded-md border-2 border-black bg-white px-4 py-2 text-xs font-extrabold text-black shadow-2xs hover:bg-black/5 transition-colors disabled:opacity-50"
                        >
                            Cancel
                        </button>
                    )}

                    <button
                        type="submit"
                        disabled={isSubmitting}
                        className="flex items-center gap-1.5 rounded-md border-2 border-black bg-amber-400 px-5 py-2 text-xs font-extrabold text-black shadow-2xs hover:bg-amber-300 transition-colors disabled:opacity-50"
                    >
                        {isSubmitting ? (
                            <Loader2 className="h-4 w-4 animate-spin text-black" />
                        ) : (
                            <Save className="h-4 w-4 text-black" />
                        )}
                        {isEditing ? 'Save Changes' : 'Create Checkpoint'}
                    </button>
                </div>
            </form>
        </div>
    )
}
