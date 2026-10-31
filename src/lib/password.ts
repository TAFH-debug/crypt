export interface Entry {
  id: number;
  site: string;
  username: string;
  password: string;
  notes: string;
  date?: string;
}

/** A file stored in the vault. Contents stay in Rust; only metadata crosses over. */
export interface StoredFile {
  id: number;
  name: string;
  size: number;
  sha256: string;
  /** Unix milliseconds. */
  added: number;
  entryId?: number;
}

export interface ImportSummary {
  entries: Entry[];
  files: StoredFile[];
  addedEntries: number;
  addedFiles: number;
  skipped: number;
}

export interface VaultStatus {
  exists: boolean;
  recoveryQuestion: string | null;
}

export const MIN_PASSWORD_LENGTH = 8;

export const REVEAL_MS = 10_000;
export const CLIPBOARD_CLEAR_MS = 30_000;
export const AUTO_LOCK_MS = 5 * 60_000;

const CHARSET = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+';

export function generatePassword(length = 20): string {
  const values = crypto.getRandomValues(new Uint32Array(length));
  return Array.from(values, (v) => CHARSET[v % CHARSET.length]).join('');
}

export function filterEntries(entries: Entry[], query: string): Entry[] {
  const q = query.trim().toLowerCase();
  return entries
    .filter((e) => !q || [e.site, e.username, e.notes].some((f) => f.toLowerCase().includes(q)))
    .sort((a, b) => a.site.localeCompare(b.site));
}

/** Score 0-4 from length and character variety. -1 means empty. */
export function strength(password: string): { score: number; label: string } {
  if (!password) return { score: -1, label: '' };
  let score = 0;
  if (password.length >= 8) score++;
  if (password.length >= 14) score++;
  if (/[a-z]/.test(password) && /[A-Z]/.test(password) && /\d/.test(password)) score++;
  if (/[^A-Za-z0-9]/.test(password)) score++;
  const labels = ['Weak', 'Weak', 'Fair', 'Good', 'Strong'];
  return { score, label: labels[score] };
}


/** Drawer letter a site is filed under: A–Z, or # for anything else. */
export function fileLetter(site: string): string {
  const ch = site.trim().charAt(0).toUpperCase();
  return ch >= 'A' && ch <= 'Z' ? ch : '#';
}

export function formatDate(when: string | number): string {
  return new Date(when).toLocaleDateString('en-GB', { day: 'numeric', month: 'short', year: 'numeric' });
}

export function formatBytes(n: number): string {
  if (n < 1024) return `${n} B`;
  const units = ['KB', 'MB', 'GB'];
  let v = n / 1024;
  let i = 0;
  while (v >= 1024 && i < units.length - 1) {
    v /= 1024;
    i++;
  }
  return `${v < 10 ? v.toFixed(1) : Math.round(v)} ${units[i]}`;
}

export function plural(n: number, word: string): string {
  return `${n} ${word}${n === 1 ? '' : 's'}`;
}

/** Last path segment, for showing a picked file. */
export function baseName(path: string): string {
  return path.split(/[\\/]/).pop() ?? path;
}

/** Drawer letters in the order their guide cards appear. Entries must be sorted. */
export function fileLetters(entries: Entry[]): string[] {
  const out: string[] = [];
  for (const e of entries) {
    const letter = fileLetter(e.site);
    if (out.at(-1) !== letter) out.push(letter);
  }
  return out;
}
