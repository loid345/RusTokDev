'use client';

import { useCallback } from 'react';

import { useLocaleStore } from '@/shared/lib/i18n';
import type { Locale } from '@/shared/lib/i18n';

import en from '../../../messages/en.json';
import ru from '../../../messages/ru.json';

const messages: Record<Locale, Record<string, string>> = { en, ru };

/**
 * React hook for translations in client components.
 * Mirrors the Leptos admin `translate()` / `use_locale()` pattern.
 */
export function useT() {
  const locale = useLocaleStore((s) => s.locale);
  const setLocale = useLocaleStore((s) => s.setLocale);

  const t = useCallback(
    (key: string): string => {
      return messages[locale]?.[key] ?? key;
    },
    [locale]
  );

  return { t, locale, setLocale };
}
