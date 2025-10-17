import axios, { AxiosInstance, AxiosResponse, AxiosError, InternalAxiosRequestConfig } from 'axios';
import { 
  ApiConfig,
  HimsApiError,
  AuthToken,
} from '@open-hims/types';

// Extended config interfaces to add custom properties
interface ExtendedAxiosRequestConfig extends InternalAxiosRequestConfig {
  metadata?: { startTime: number };
  retryCount?: number;
}

// Enhanced API client with better Axios control
export class HimsApiClient {
  private client: AxiosInstance;
  private config: ApiConfig;

  constructor(config: ApiConfig = {}) {
    // Browser-compatible environment variable access
    const getEnvVar = (key: string, defaultValue: string = ''): string => {
      // Node.js environment
      if (typeof process !== 'undefined' && process.env) {
        return process.env[key] || defaultValue;
      }
      
      // Browser environment - check for Vite env vars
      if (typeof window !== 'undefined') {
        const viteEnv = (window as any).__VITE_ENV__;
        if (viteEnv && viteEnv[key]) {
          return viteEnv[key];
        }
        
        // Check for import.meta.env (Vite) - use try/catch for safety
        try {
          if (typeof import.meta !== 'undefined') {
            const metaEnv = (import.meta as any).env;
            if (metaEnv && metaEnv[key]) {
              return metaEnv[key] || defaultValue;
            }
          }
        } catch {
          // Ignore import.meta errors in non-Vite environments
        }
      }
      
      return defaultValue;
    };

    const isDevelopment = (): boolean => {
      // Node.js environment
      if (typeof process !== 'undefined' && process.env) {
        return process.env.NODE_ENV === 'development';
      }
      
      // Browser environment - check for development indicators
      if (typeof window !== 'undefined') {
        // Check hostname for localhost
        if (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1') {
          return true;
        }
        
        // Check for Vite dev mode - use try/catch for safety
        try {
          if (typeof import.meta !== 'undefined') {
            const metaEnv = (import.meta as any).env;
            return metaEnv && metaEnv.DEV === true;
          }
        } catch {
          // Ignore import.meta errors in non-Vite environments
        }
      }
      
      return false;
    };

    this.config = {
      baseURL: getEnvVar('HIMS_API_BASE_URL', 'http://localhost:8000/api'),
      timeout: 30000,
      retries: 3,
      retryDelay: 1000,
      enableLogging: isDevelopment(),
      enableRetry: true,
      headers: {
        'Content-Type': 'application/json',
      },
      ...config,
    };

    this.client = axios.create(this.config);
    this.setupInterceptors();
  }

  private setupInterceptors(): void {
    // Request interceptor with auth and logging
    this.client.interceptors.request.use(
      (config: ExtendedAxiosRequestConfig) => {
        // Add auth token if available
        const token = this.getAuthToken();
        if (token) {
          config.headers = config.headers || {};
          config.headers.Authorization = `Bearer ${typeof token === 'string' ? token : token.accessToken}`;
        }

        // Add request timestamp
        config.metadata = { startTime: Date.now() };

        // Log request if enabled
        if (this.config.enableLogging) {
          console.log(`🔄 API Request: ${config.method?.toUpperCase()} ${config.url}`, {
            params: config.params,
            data: config.data,
          });
        }

        return config;
      },
      (error: AxiosError) => {
        if (this.config.enableLogging) {
          console.error('❌ API Request Error:', error);
        }
        return Promise.reject(this.createApiError(error));
      }
    );

    // Response interceptor with retry logic and error handling
    this.client.interceptors.response.use(
      (response: AxiosResponse) => {
        // Calculate response time
        const extendedConfig = response.config as ExtendedAxiosRequestConfig;
        const responseTime = Date.now() - (extendedConfig.metadata?.startTime || 0);

        // Log response if enabled
        if (this.config.enableLogging) {
          console.log(`✅ API Response: ${response.status} ${response.config.url} (${responseTime}ms)`, {
            data: response.data,
            headers: response.headers,
          });
        }

        return response;
      },
      async (error: AxiosError) => {
        const config = error.config as ExtendedAxiosRequestConfig;

        // Retry logic for network errors or 5xx responses
        if (this.shouldRetry(error) && config && this.config.enableRetry) {
          config.retryCount = config.retryCount || 0;
          
          if (config.retryCount < (this.config.retries || 3)) {
            config.retryCount++;
            
            // Wait before retrying
            await this.delay(this.config.retryDelay! * config.retryCount);
            
            if (this.config.enableLogging) {
              console.log(`🔄 Retrying API call (attempt ${config.retryCount}):`, config.url);
            }
            
            return this.client(config);
          }
        }

        // Handle 401 unauthorized - clear token and potentially redirect
        if (error.response?.status === 401) {
          this.clearAuthToken();
        }

        const apiError = this.createApiError(error);
        
        if (this.config.enableLogging) {
          console.error('❌ API Error:', apiError);
        }

        return Promise.reject(apiError);
      }
    );
  }

  private shouldRetry(error: AxiosError): boolean {
    if (!error.config) return false;
    
    // Retry on network errors
    if (!error.response) return true;
    
    // Retry on 5xx server errors
    if (error.response.status >= 500) return true;
    
    // Retry on specific 4xx errors (rate limiting, etc.)
    if (error.response.status === 429) return true;
    
    return false;
  }

  private delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  private createApiError(error: AxiosError): HimsApiError {
    return {
      code: error.code || 'UNKNOWN_ERROR',
      message: error.message,
      status: error.response?.status,
      timestamp: new Date().toISOString(),
      details: error.response?.data as Record<string, any> | undefined,
      originalError: error,
      config: error.config,
      response: error.response,
    };
  }

  private getAuthToken(): AuthToken | string | null {
    // Get token from localStorage/sessionStorage/secure storage
    if (typeof window !== 'undefined') {
      const tokenStr = localStorage.getItem('hims_auth_token');
      if (tokenStr) {
        try {
          // Try to parse as AuthToken object first
          return JSON.parse(tokenStr) as AuthToken;
        } catch {
          // Fall back to string token
          return tokenStr;
        }
      }
    }
    return null;
  }

  private clearAuthToken(): void {
    if (typeof window !== 'undefined') {
      localStorage.removeItem('hims_auth_token');
    }
  }

  // Generic request method with proper typing
  async request<T>(
    method: 'get' | 'post' | 'put' | 'delete',
    url: string,
    data?: any,
    params?: any
  ): Promise<T> {
    const response = await this.client({
      method,
      url,
      data,
      params,
    });
    return response.data;
  }
}

// Default API client instance
export const apiClient = new HimsApiClient();