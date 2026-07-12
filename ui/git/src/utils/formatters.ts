// =============================================================================
// Tardigrade-CI Git Module - Formatters Utilities
// =============================================================================

// ==----------------------------------------------------------------------------
// Formatage des dates
// ==----------------------------------------------------------------------------

/**
 * Formate une date ISO en chaîne lisible
 * @param date - Date au format ISO (ex: "2024-01-15T10:30:00Z")
 * @param options - Options de formatage
 * @returns Date formatée
 */
export function formatDate(date: string, options?: Intl.DateTimeFormatOptions): string {
  const defaultOptions: Intl.DateTimeFormatOptions = {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  };

  const finalOptions = { ...defaultOptions, ...options };

  try {
    const dateObj = new Date(date);
    return dateObj.toLocaleDateString('fr-FR', finalOptions);
  } catch {
    return date;
  }
}

/**
 * Formate une date relative (ex: "Il y a 2 heures")
 */
export function formatRelativeDate(date: string): string {
  try {
    const dateObj = new Date(date);
    const now = new Date();
    const diffInSeconds = Math.floor((now.getTime() - dateObj.getTime()) / 1000);

    const intervals = {
      année: 31536000,
      mois: 2592000,
      semaine: 604800,
      jour: 86400,
      heure: 3600,
      minute: 60,
      seconde: 1,
    };

    for (const [unit, seconds] of Object.entries(intervals)) {
      const interval = Math.floor(diffInSeconds / seconds);
      if (interval >= 1) {
        return `Il y a ${interval} ${unit}${interval > 1 ? 's' : ''}`;
      }
    }

    return 'À l\'instant';
  } catch {
    return date;
  }
}

/**
 * Formate une date au format "JJ/MM/AAAA"
 */
export function formatShortDate(date: string): string {
  try {
    const dateObj = new Date(date);
    const day = dateObj.getDate().toString().padStart(2, '0');
    const month = (dateObj.getMonth() + 1).toString().padStart(2, '0');
    const year = dateObj.getFullYear();
    return `${day}/${month}/${year}`;
  } catch {
    return date;
  }
}

/**
 * Formate une date au format ISO pour l'API
 */
export function formatIsoDate(date: Date): string {
  return date.toISOString();
}

// ==----------------------------------------------------------------------------
// Formatage des nombres
// ==----------------------------------------------------------------------------

/**
 * Formate un nombre avec séparateurs de milliers
 */
export function formatNumber(value: number): string {
  return new Intl.NumberFormat('fr-FR').format(value);
}

/**
 * Formate un nombre en pourcentage
 */
export function formatPercentage(value: number, decimals: number = 1): string {
  const multiplier = Math.pow(10, decimals);
  const rounded = Math.round(value * multiplier) / multiplier;
  return `${rounded}%`;
}

/**
 * Formate une taille en octets en taille lisible (KB, MB, GB)
 */
export function formatBytes(bytes: number, decimals: number = 2): string {
  if (bytes === 0) return '0 octets';

  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ['octets', 'Ko', 'Mo', 'Go', 'To', 'Po'];

  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
}

// ==----------------------------------------------------------------------------
// Formatage des identifiants
// ==----------------------------------------------------------------------------

/**
 * Racourcit un hash ou un ID long
 */
export function shortenHash(hash: string, length: number = 8): string {
  if (!hash) return '';
  if (hash.length <= length * 2) return hash;
  return `${hash.substring(0, length)}...${hash.substring(hash.length - length)}`;
}

/**
 * Met en majuscule la première lettre d'une chaîne
 */
export function capitalize(str: string): string {
  if (!str) return '';
  return str.charAt(0).toUpperCase() + str.slice(1);
}

/**
 * Génère des initiales à partir d'une chaîne
 */
export function getInitials(str: string, maxLength: number = 2): string {
  if (!str) return '?';
  
  const words = str.trim().split(/\s+/);
  let initials = '';
  
  for (const word of words) {
    if (word && initials.length < maxLength) {
      initials += word.charAt(0).toUpperCase();
    }
  }
  
  return initials || '?';
}

// ==----------------------------------------------------------------------------
// Formatage des URLs
// ==----------------------------------------------------------------------------

/**
 * Extrait le nom du domaine d'une URL
 */
export function extractDomain(url: string): string {
  try {
    const domain = new URL(url).hostname;
    return domain.replace('www.', '');
  } catch {
    return url;
  }
}

/**
 * Vérifie si une URL est valide
 */
export function isValidUrl(url: string): boolean {
  try {
    new URL(url);
    return true;
  } catch {
    return false;
  }
}

// ==----------------------------------------------------------------------------
// Formatage des durées
// ==----------------------------------------------------------------------------

/**
 * Formate une durée en millisecondes en chaîne lisible
 */
export function formatDuration(ms: number): string {
  if (ms < 1000) return `${ms}ms`;
  
  const seconds = Math.floor(ms / 1000);
  if (seconds < 60) return `${seconds}s`;
  
  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) return `${minutes}m`;
  
  const hours = Math.floor(minutes / 60);
  return `${hours}h`;
}

/**
 * Formate une durée au format "HH:MM:SS"
 */
export function formatTime(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;
  
  return [h, m, s]
    .map(v => v.toString().padStart(2, '0'))
    .join(':');
}

// ==----------------------------------------------------------------------------
// Tests unitaires
// ==----------------------------------------------------------------------------

if (import.meta.vitest) {
  const { describe, it, expect } = import.meta.vitest;

  describe('formatters', () => {
    it('should format date', () => {
      const result = formatDate('2024-01-15T10:30:00Z');
      expect(result).toContain('15');
      expect(result).toContain('janv');
    });

    it('should format bytes', () => {
      expect(formatBytes(1024)).toBe('1 Ko');
      expect(formatBytes(1024 * 1024)).toBe('1 Mo');
    });

    it('should shorten hash', () => {
      const hash = 'abcdefghijklmnopqrstuvwxyz1234567890';
      expect(shortenHash(hash, 4)).toBe('abcd...567890');
    });

    it('should capitalize', () => {
      expect(capitalize('hello')).toBe('Hello');
      expect(capitalize('hello world')).toBe('Hello world');
    });

    it('should get initials', () => {
      expect(getInitials('John Doe')).toBe('JD');
      expect(getInitials('John Doe Smith')).toBe('JD');
    });
  });
}
