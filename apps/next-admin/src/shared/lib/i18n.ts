import { create } from 'zustand';

import en from '../../../messages/en.json';
import ru from '../../../messages/ru.json';

export type Locale = 'en' | 'ru';

const messages: Record<Locale, Record<string, string>> = { en, ru };

interface LocaleState {
  locale: Locale;
  setLocale: (locale: Locale) => void;
}

function getInitialLocale(): Locale {
  if (typeof window === 'undefined') return 'en';
  const stored = localStorage.getItem('rustok-admin-locale');
  if (stored === 'ru' || stored === 'en') return stored;
  return 'en';
}

export const useLocaleStore = create<LocaleState>((set) => ({
  locale: getInitialLocale(),
  setLocale: (locale: Locale) => {
    if (typeof window !== 'undefined') {
      localStorage.setItem('rustok-admin-locale', locale);
    }
    set({ locale });
  }
}));

/**
 * Translate a key using the current locale.
 * Falls back to the key itself if not found.
 */
export function t(key: string, locale?: Locale): string {
  const l = locale ?? useLocaleStore.getState().locale;
  return messages[l]?.[key] ?? key;
}
