// HTTP Client Wrapper for OpenAgentMail

import type { OpenAgentMailConfig, RequestOptions, ApiErrorResponse } from '../types.js';
import {
  APIError,
  BadRequestError,
  AuthenticationError,
  AuthorizationError,
  NotFoundError,
  ConflictError,
  ValidationError,
  RateLimitError,
  InternalServerError,
  TimeoutError,
  NetworkError,
} from '../errors.js';

const DEFAULT_BASE_URL = 'https://api.openagentmail.com/v0';
const DEFAULT_TIMEOUT = 30000;

/**
 * Convert camelCase to snake_case for API requests
 */
export function toSnakeCase(str: string): string {
  return str.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`);
}

/**
 * Convert snake_case to camelCase for API responses
 */
export function toCamelCase(str: string): string {
  return str.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
}

/**
 * Recursively transform object keys from camelCase to snake_case
 */
export function transformRequestBody(obj: unknown): unknown {
  if (obj === null || obj === undefined) {
    return obj;
  }
  
  if (Array.isArray(obj)) {
    return obj.map(transformRequestBody);
  }
  
  if (typeof obj === 'object') {
    const result: Record<string, unknown> = {};
    for (const [key, value] of Object.entries(obj as Record<string, unknown>)) {
      result[toSnakeCase(key)] = transformRequestBody(value);
    }
    return result;
  }
  
  return obj;
}

/**
 * Recursively transform object keys from snake_case to camelCase
 */
export function transformResponseBody<T>(obj: unknown): T {
  if (obj === null || obj === undefined) {
    return obj as T;
  }
  
  if (Array.isArray(obj)) {
    return obj.map(transformResponseBody) as T;
  }
  
  if (typeof obj === 'object') {
    const result: Record<string, unknown> = {};
    for (const [key, value] of Object.entries(obj as Record<string, unknown>)) {
      result[toCamelCase(key)] = transformResponseBody(value);
    }
    return result as T;
  }
  
  return obj as T;
}

/**
 * Build query string from parameters
 */
function buildQueryString(params: Record<string, string | number | boolean | undefined>): string {
  const searchParams = new URLSearchParams();
  
  for (const [key, value] of Object.entries(params)) {
    if (value !== undefined) {
      searchParams.set(toSnakeCase(key), String(value));
    }
  }
  
  const queryString = searchParams.toString();
  return queryString ? `?${queryString}` : '';
}

/**
 * HTTP client for making API requests
 */
export class HttpClient {
  private readonly apiKey: string;
  private readonly baseUrl: string;
  private readonly timeout: number;
  private readonly fetchFn: typeof fetch;

  constructor(config: OpenAgentMailConfig) {
    this.apiKey = config.apiKey;
    this.baseUrl = config.baseUrl ?? DEFAULT_BASE_URL;
    this.timeout = config.timeout ?? DEFAULT_TIMEOUT;
    this.fetchFn = config.fetch ?? fetch;
  }

  /**
   * Make an HTTP request to the API
   */
  async request<T>(options: RequestOptions): Promise<T> {
    const { method, path, body, query } = options;
    
    let url = `${this.baseUrl}${path}`;
    if (query) {
      url += buildQueryString(query);
    }

    const headers: Record<string, string> = {
      'Authorization': `Bearer ${this.apiKey}`,
      'Content-Type': 'application/json',
      'Accept': 'application/json',
    };

    const requestInit: RequestInit = {
      method,
      headers,
    };

    if (body !== undefined) {
      requestInit.body = JSON.stringify(transformRequestBody(body));
    }

    // Create abort controller for timeout
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.timeout);
    requestInit.signal = controller.signal;

    let response: Response;
    
    try {
      response = await this.fetchFn(url, requestInit);
    } catch (error) {
      clearTimeout(timeoutId);
      
      if (error instanceof Error) {
        if (error.name === 'AbortError') {
          throw new TimeoutError(this.timeout);
        }
        throw new NetworkError(`Network request failed: ${error.message}`, error);
      }
      
      throw new NetworkError('Network request failed');
    } finally {
      clearTimeout(timeoutId);
    }

    // Handle 204 No Content
    if (response.status === 204) {
      return undefined as T;
    }

    let responseBody: unknown;
    
    try {
      responseBody = await response.json();
    } catch {
      if (!response.ok) {
        throw new APIError(response.status, {
          code: 'unknown_error',
          message: `Request failed with status ${response.status}`,
        });
      }
      return undefined as T;
    }

    if (!response.ok) {
      const errorResponse = responseBody as ApiErrorResponse;
      const error = errorResponse.error ?? {
        code: 'unknown_error',
        message: `Request failed with status ${response.status}`,
      };

      switch (response.status) {
        case 400:
          throw new BadRequestError(error);
        case 401:
          throw new AuthenticationError(error);
        case 403:
          throw new AuthorizationError(error);
        case 404:
          throw new NotFoundError(error);
        case 409:
          throw new ConflictError(error);
        case 422:
          throw new ValidationError(error);
        case 429:
          throw new RateLimitError(error, response.headers);
        default:
          if (response.status >= 500) {
            throw new InternalServerError(response.status, error);
          }
          throw new APIError(response.status, error);
      }
    }

    return transformResponseBody<T>(responseBody);
  }

  /**
   * Make a GET request
   */
  get<T>(path: string, query?: Record<string, string | number | boolean | undefined>): Promise<T> {
    return this.request<T>({ method: 'GET', path, query });
  }

  /**
   * Make a POST request
   */
  post<T>(path: string, body?: unknown): Promise<T> {
    return this.request<T>({ method: 'POST', path, body });
  }

  /**
   * Make a PUT request
   */
  put<T>(path: string, body?: unknown): Promise<T> {
    return this.request<T>({ method: 'PUT', path, body });
  }

  /**
   * Make a PATCH request
   */
  patch<T>(path: string, body?: unknown): Promise<T> {
    return this.request<T>({ method: 'PATCH', path, body });
  }

  /**
   * Make a DELETE request
   */
  delete<T>(path: string): Promise<T> {
    return this.request<T>({ method: 'DELETE', path });
  }
}
