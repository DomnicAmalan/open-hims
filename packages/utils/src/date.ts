import { format, parseISO, isValid, addDays, subDays, differenceInDays, differenceInYears } from 'date-fns';

// Date formatting utilities
export function formatDate(date: string | Date, formatString: string = 'yyyy-MM-dd'): string {
  const dateObj = typeof date === 'string' ? parseISO(date) : date;
  if (!isValid(dateObj)) {
    throw new Error('Invalid date provided');
  }
  return format(dateObj, formatString);
}

export function formatDateTime(date: string | Date, formatString: string = 'yyyy-MM-dd HH:mm:ss'): string {
  return formatDate(date, formatString);
}

export function formatTime(date: string | Date, formatString: string = 'HH:mm'): string {
  return formatDate(date, formatString);
}

// Date validation
export function isValidDate(date: string | Date): boolean {
  const dateObj = typeof date === 'string' ? parseISO(date) : date;
  return isValid(dateObj);
}

export function isValidDateString(dateString: string): boolean {
  if (!dateString) return false;
  const date = parseISO(dateString);
  return isValid(date);
}

// Date calculations
export function addDaysToDate(date: string | Date, days: number): Date {
  const dateObj = typeof date === 'string' ? parseISO(date) : date;
  if (!isValid(dateObj)) {
    throw new Error('Invalid date provided');
  }
  return addDays(dateObj, days);
}

export function subtractDaysFromDate(date: string | Date, days: number): Date {
  const dateObj = typeof date === 'string' ? parseISO(date) : date;
  if (!isValid(dateObj)) {
    throw new Error('Invalid date provided');
  }
  return subDays(dateObj, days);
}

export function getDaysBetween(startDate: string | Date, endDate: string | Date): number {
  const start = typeof startDate === 'string' ? parseISO(startDate) : startDate;
  const end = typeof endDate === 'string' ? parseISO(endDate) : endDate;
  
  if (!isValid(start) || !isValid(end)) {
    throw new Error('Invalid dates provided');
  }
  
  return differenceInDays(end, start);
}

export function getAge(birthDate: string | Date, referenceDate?: string | Date): number {
  const birth = typeof birthDate === 'string' ? parseISO(birthDate) : birthDate;
  const reference = referenceDate 
    ? (typeof referenceDate === 'string' ? parseISO(referenceDate) : referenceDate)
    : new Date();
  
  if (!isValid(birth) || !isValid(reference)) {
    throw new Error('Invalid dates provided');
  }
  
  return differenceInYears(reference, birth);
}

// ISO date utilities
export function toISODate(date: Date): string {
  return date.toISOString().split('T')[0];
}

export function toISODateTime(date: Date): string {
  return date.toISOString();
}

export function fromISODate(isoString: string): Date {
  const date = parseISO(isoString);
  if (!isValid(date)) {
    throw new Error('Invalid ISO date string');
  }
  return date;
}

// Healthcare-specific date utilities
export function formatDateOfBirth(birthDate: string | Date): string {
  return formatDate(birthDate, 'MMM dd, yyyy');
}

export function isAdult(birthDate: string | Date, adultAge: number = 18): boolean {
  return getAge(birthDate) >= adultAge;
}

export function isMinor(birthDate: string | Date, adultAge: number = 18): boolean {
  return getAge(birthDate) < adultAge;
}

export function getAgeGroup(birthDate: string | Date): 'infant' | 'child' | 'adolescent' | 'adult' | 'senior' {
  const age = getAge(birthDate);
  
  if (age < 2) return 'infant';
  if (age < 13) return 'child';
  if (age < 18) return 'adolescent';
  if (age < 65) return 'adult';
  return 'senior';
}

// Appointment and scheduling utilities
export function isBusinessDay(date: string | Date): boolean {
  const dateObj = typeof date === 'string' ? parseISO(date) : date;
  const dayOfWeek = dateObj.getDay();
  return dayOfWeek >= 1 && dayOfWeek <= 5; // Monday to Friday
}

export function getNextBusinessDay(date: string | Date): Date {
  let nextDay = addDays(typeof date === 'string' ? parseISO(date) : date, 1);
  while (!isBusinessDay(nextDay)) {
    nextDay = addDays(nextDay, 1);
  }
  return nextDay;
}

export function getPreviousBusinessDay(date: string | Date): Date {
  let prevDay = subDays(typeof date === 'string' ? parseISO(date) : date, 1);
  while (!isBusinessDay(prevDay)) {
    prevDay = subDays(prevDay, 1);
  }
  return prevDay;
}

// Time zone utilities
export function convertToUTC(date: string | Date): Date {
  const dateObj = typeof date === 'string' ? parseISO(date) : date;
  return new Date(dateObj.getTime() + (dateObj.getTimezoneOffset() * 60000));
}

export function convertFromUTC(utcDate: string | Date, timezoneOffset: number): Date {
  const dateObj = typeof utcDate === 'string' ? parseISO(utcDate) : utcDate;
  return new Date(dateObj.getTime() - (timezoneOffset * 60000));
}