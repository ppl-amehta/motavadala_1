import { writable } from 'svelte/store';

// Define the type for our user object
export interface User {
  id: string;
  name: string;
  email: string;
  role: string;
  // Add any other user-specific fields you expect from the backend
}

// Define the type for our auth state
export interface AuthState {
  isAuthenticated: boolean;
  user: User | null;
  token: string | null;
}

// Initial state
const initialAuthState: AuthState = {
  isAuthenticated: false,
  user: null,
  token: null,
};

// Create a writable store
const authStore = writable<AuthState>(initialAuthState);

// Function to set auth data (e.g., after login)
export function setAuth(userData: User, authToken: string) {
  authStore.set({
    isAuthenticated: true,
    user: userData,
    token: authToken,
  });
  // TODO: Persist token to localStorage for session persistence
  if (typeof localStorage !== 'undefined') {
    localStorage.setItem('authToken', authToken);
    localStorage.setItem('authUser', JSON.stringify(userData));
  }
}

// Function to clear auth data (e.g., after logout)
export function clearAuth() {
  authStore.set(initialAuthState);
  // TODO: Remove token from localStorage
  if (typeof localStorage !== 'undefined') {
    localStorage.removeItem('authToken');
    localStorage.removeItem('authUser');
  }
}

// Function to initialize auth state from localStorage (e.g., on app load)
export function initializeAuth() {
  if (typeof localStorage !== 'undefined') {
    const token = localStorage.getItem('authToken');
    const storedUser = localStorage.getItem('authUser');
    if (token && storedUser) {
      try {
        const user: User = JSON.parse(storedUser);
        authStore.set({
          isAuthenticated: true,
          user: user,
          token: token,
        });
      } catch (e) {
        console.error("Error parsing stored user data:", e);
        clearAuth(); // Clear potentially corrupted data
      }
    }
  }
}

export default authStore;

