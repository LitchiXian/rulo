export interface ProfileDecorState {
  avatar: string
  cover: string
}

const STORAGE_KEY = 'admin-profile-decor'
export const PROFILE_DECOR_CHANGE_EVENT = 'admin-profile-decor-change'

const defaultState: ProfileDecorState = {
  avatar: '',
  cover: '',
}

export function loadProfileDecor(): ProfileDecorState {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) return { ...defaultState }
    return { ...defaultState, ...JSON.parse(raw) }
  } catch {
    return { ...defaultState }
  }
}

export function saveProfileDecor(partial: Partial<ProfileDecorState>) {
  const next = { ...loadProfileDecor(), ...partial }
  localStorage.setItem(STORAGE_KEY, JSON.stringify(next))
  window.dispatchEvent(new CustomEvent(PROFILE_DECOR_CHANGE_EVENT, { detail: next }))
  return next
}

export function clearProfileDecor<K extends keyof ProfileDecorState>(key: K) {
  return saveProfileDecor({ [key]: '' } as Pick<ProfileDecorState, K>)
}