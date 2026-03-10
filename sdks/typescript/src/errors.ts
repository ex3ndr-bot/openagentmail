// OpenAgentMail Error Classes

import type { ApiError } from './types.js';

/**
 * Base error class for all OpenAgentMail errors
 */
export class OpenAgentMailError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'OpenAgentMailError';
    Object.setPrototypeOf(this, OpenAgentMailError.prototype);
  }
}

/**
 * Error thrown when the API returns an error response
 */
export class APIError extends OpenAgentMailError {
  /** HTTP status code */
  readonly status: number;
  /** Error code from the API */
  readonly code: string;
  /** Additional error details */
  readonly details?: Record<string, unknown>;

  constructor(status: number, error: ApiError) {
    super(error.message);
    this.name = 'APIError';
    this.status = status;
    this.code = error.code;
    this.details = error.details;
    Object.setPrototypeOf(this, APIError.prototype);
  }
}

/**
 * Error thrown for 400 Bad Request responses
 */
export class BadRequestError extends APIError {
  constructor(error: ApiError) {
    super(400, error);
    this.name = 'BadRequestError';
    Object.setPrototypeOf(this, BadRequestError.prototype);
  }
}

/**
 * Error thrown for 401 Unauthorized responses
 */
export class AuthenticationError extends APIError {
  constructor(error: ApiError) {
    super(401, error);
    this.name = 'AuthenticationError';
    Object.setPrototypeOf(this, AuthenticationError.prototype);
  }
}

/**
 * Error thrown for 403 Forbidden responses
 */
export class AuthorizationError extends APIError {
  constructor(error: ApiError) {
    super(403, error);
    this.name = 'AuthorizationError';
    Object.setPrototypeOf(this, AuthorizationError.prototype);
  }
}

/**
 * Error thrown for 404 Not Found responses
 */
export class NotFoundError extends APIError {
  constructor(error: ApiError) {
    super(404, error);
    this.name = 'NotFoundError';
    Object.setPrototypeOf(this, NotFoundError.prototype);
  }
}

/**
 * Error thrown for 409 Conflict responses (idempotency)
 */
export class ConflictError extends APIError {
  constructor(error: ApiError) {
    super(409, error);
    this.name = 'ConflictError';
    Object.setPrototypeOf(this, ConflictError.prototype);
  }
}

/**
 * Error thrown for 422 Validation Error responses
 */
export class ValidationError extends APIError {
  constructor(error: ApiError) {
    super(422, error);
    this.name = 'ValidationError';
    Object.setPrototypeOf(this, ValidationError.prototype);
  }
}

/**
 * Error thrown for 429 Rate Limited responses
 */
export class RateLimitError extends APIError {
  /** Unix timestamp when the rate limit resets */
  readonly resetAt?: number;
  /** Number of seconds until the rate limit resets */
  readonly retryAfter?: number;

  constructor(error: ApiError, headers?: Headers) {
    super(429, error);
    this.name = 'RateLimitError';
    
    if (headers) {
      const reset = headers.get('X-RateLimit-Reset');
      if (reset) {
        this.resetAt = parseInt(reset, 10);
      }
      const retryAfter = headers.get('Retry-After');
      if (retryAfter) {
        this.retryAfter = parseInt(retryAfter, 10);
      }
    }
    
    Object.setPrototypeOf(this, RateLimitError.prototype);
  }
}

/**
 * Error thrown for 500+ Internal Server Error responses
 */
export class InternalServerError extends APIError {
  constructor(status: number, error: ApiError) {
    super(status, error);
    this.name = 'InternalServerError';
    Object.setPrototypeOf(this, InternalServerError.prototype);
  }
}

/**
 * Error thrown when a request times out
 */
export class TimeoutError extends OpenAgentMailError {
  constructor(timeout: number) {
    super(`Request timed out after ${timeout}ms`);
    this.name = 'TimeoutError';
    Object.setPrototypeOf(this, TimeoutError.prototype);
  }
}

/**
 * Error thrown when there's a network error
 */
export class NetworkError extends OpenAgentMailError {
  readonly cause?: Error;

  constructor(message: string, cause?: Error) {
    super(message);
    this.name = 'NetworkError';
    this.cause = cause;
    Object.setPrototypeOf(this, NetworkError.prototype);
  }
}
