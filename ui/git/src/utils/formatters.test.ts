// =============================================================================
// Tests unitaires - utilitaires de formatage (formatters)
// =============================================================================

import { describe, it, expect } from 'vitest';
import {
  formatDate,
  formatShortDate,
  formatNumber,
  formatPercentage,
  formatBytes,
  shortenHash,
  capitalize,
  getInitials,
  isValidUrl,
  getApiErrorMessage,
} from './formatters';

describe('formatDate', () => {
  it('formate une date ISO valide', () => {
    const result = formatDate('2024-01-15T10:30:00Z');
    expect(result).toContain('2024');
    expect(result.length).toBeGreaterThan(0);
  });

  it('produit une chaîne pour une date invalide', () => {
    // `new Date('not-a-date')` ne lance pas mais produit une date invalide ;
    // le résultat n'est pas vide.
    const result = formatDate('not-a-date');
    expect(typeof result).toBe('string');
    expect(result.length).toBeGreaterThan(0);
  });
});

describe('formatShortDate', () => {
  it('formate une date ISO en date courte', () => {
    const result = formatShortDate('2024-01-15T10:30:00Z');
    expect(result).toContain('2024');
  });
});

describe('formatNumber', () => {
  it('formate un nombre entier avec séparateurs', () => {
    expect(formatNumber(1000000)).toContain('1');
  });

  it('formate zéro', () => {
    expect(formatNumber(0)).toContain('0');
  });
});

describe('formatPercentage', () => {
  it('calcule un pourcentage', () => {
    const result = formatPercentage(50, 100);
    expect(result).toContain('50');
  });
});

describe('formatBytes', () => {
  it('formate 0 octet', () => {
    expect(formatBytes(0)).toContain('0');
  });

  it('formate des kilo-octets', () => {
    expect(formatBytes(1024)).toContain('1');
  });

  it('formate des méga-octets', () => {
    expect(formatBytes(1024 * 1024)).toContain('1');
  });
});

describe('shortenHash', () => {
  it('raccourcit un hash long', () => {
    const hash = 'abcdefghijklmnopqrstuvwxyz1234567890';
    const result = shortenHash(hash, 4);
    expect(result).toContain('...');
    expect(result.length).toBeLessThan(hash.length);
  });

  it('retourne le hash tel quel s’il est plus court que la limite', () => {
    expect(shortenHash('abc', 4)).toBe('abc');
  });
});

describe('capitalize', () => {
  it('met la première lettre en majuscule', () => {
    expect(capitalize('hello')).toBe('Hello');
  });

  it('gère une chaîne vide', () => {
    expect(capitalize('')).toBe('');
  });
});

describe('getInitials', () => {
  it('retourne les initiales d’un nom composé', () => {
    expect(getInitials('John Doe')).toBe('JD');
  });

  it('limite le nombre d’initiales', () => {
    expect(getInitials('John Doe Smith')).toBe('JD');
  });
});

describe('isValidUrl', () => {
  it('valide une URL http', () => {
    expect(isValidUrl('http://example.com')).toBe(true);
  });

  it('valide une URL https', () => {
    expect(isValidUrl('https://example.com')).toBe(true);
  });

  it('invalide une chaîne non-URL', () => {
    expect(isValidUrl('not-a-url')).toBe(false);
  });
});

describe('getApiErrorMessage', () => {
  it('extrait le message d’un objet ApiError { error, status }', () => {
    const error = { error: 'Repository already exists', status: 409 };
    expect(getApiErrorMessage(error, 'fallback')).toBe('Repository already exists');
  });

  it('retourne une chaîne d’erreur brute', () => {
    expect(getApiErrorMessage('une erreur', 'fallback')).toBe('une erreur');
  });

  it('retourne le fallback pour une erreur sans champ error', () => {
    expect(getApiErrorMessage({ status: 500 }, 'fallback')).toBe('fallback');
  });

  it('retourne le fallback pour une erreur vide', () => {
    expect(getApiErrorMessage(null, 'fallback')).toBe('fallback');
    expect(getApiErrorMessage(undefined, 'fallback')).toBe('fallback');
  });

  it('retourne le fallback si error est une chaîne vide', () => {
    expect(getApiErrorMessage('', 'fallback')).toBe('fallback');
  });

  it('retourne le fallback si error.error est vide', () => {
    expect(getApiErrorMessage({ error: '', status: 400 }, 'fallback')).toBe('fallback');
  });
});
