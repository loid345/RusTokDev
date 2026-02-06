"use client";

import { useEffect } from "react";

const STORAGE_KEY = "rustok-admin-locale";
const COOKIE_NAME = "rustok-admin-locale";

type LocaleSyncProps = {
  locale: string;
};

function persistLocale(locale: string) {
  try {
    window.localStorage.setItem(STORAGE_KEY, locale);
  } catch {
    // Ignore storage errors (e.g. disabled storage).
  }

  document.cookie = `${COOKIE_NAME}=${locale}; path=/; max-age=31536000`;
}

export default function LocaleSync({ locale }: LocaleSyncProps) {
  useEffect(() => {
    if (typeof window === "undefined") {
      return;
    }

    persistLocale(locale);
  }, [locale]);

  return null;
}
