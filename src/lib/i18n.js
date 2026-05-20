// Simple i18n store for Tman
// Usage: import { t, locale, setLocale } from '$lib/i18n';
// In template: {$t('capture.title')}

import { writable, derived } from 'svelte/store';
import tr from '../i18n/tr.json';
import en from '../i18n/en.json';
import de from '../i18n/de.json';
import es from '../i18n/es.json';
import ru from '../i18n/ru.json';
import ja from '../i18n/ja.json';
import zh from '../i18n/zh.json';

/** @type {Record<string, Record<string, any>>} */
const translations = { tr, en, de, es, ru, ja, zh };

/** Current locale */
export const locale = writable('en');

/** Available locales */
export const locales = Object.keys(translations);

/**
 * Set locale and return it
 * @param {string} newLocale
 */
export function setLocale(newLocale) {
  if (translations[newLocale]) {
    locale.set(newLocale);
  }
}

/**
 * Translation function store
 * Returns a function that resolves dot-separated keys: $t('capture.title')
 */
export const t = derived(locale, ($locale) => {
  const dict = translations[$locale] || translations['tr'];
  
  /**
   * @param {string} key - Dot-separated key like 'capture.title'
   * @param {string} [fallback] - Fallback if key not found
   * @returns {string}
   */
  return (key, fallback) => {
    const parts = key.split('.');
    let val = dict;
    for (const part of parts) {
      if (val && typeof val === 'object' && part in val) {
        val = val[part];
      } else {
        return fallback ?? key;
      }
    }
    return typeof val === 'string' ? val : (fallback ?? key);
  };
});
