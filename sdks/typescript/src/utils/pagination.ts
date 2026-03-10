// Pagination Helpers for OpenAgentMail

import type { PaginatedResponse, PaginationParams } from '../types.js';

/**
 * Async iterator wrapper for paginated endpoints
 */
export class PageIterator<T> implements AsyncIterable<T> {
  private readonly fetchPage: (params: PaginationParams) => Promise<PaginatedResponse<T>>;
  private readonly initialParams: PaginationParams;

  constructor(
    fetchPage: (params: PaginationParams) => Promise<PaginatedResponse<T>>,
    initialParams: PaginationParams = {}
  ) {
    this.fetchPage = fetchPage;
    this.initialParams = initialParams;
  }

  async *[Symbol.asyncIterator](): AsyncIterator<T> {
    let pageToken: string | undefined = this.initialParams.pageToken;
    let hasMore = true;

    while (hasMore) {
      const response = await this.fetchPage({
        ...this.initialParams,
        pageToken,
      });

      for (const item of response.items) {
        yield item;
      }

      hasMore = response.hasMore;
      pageToken = response.nextPageToken ?? undefined;
    }
  }

  /**
   * Collect all items into an array
   */
  async toArray(): Promise<T[]> {
    const items: T[] = [];
    for await (const item of this) {
      items.push(item);
    }
    return items;
  }

  /**
   * Collect up to n items into an array
   */
  async take(n: number): Promise<T[]> {
    const items: T[] = [];
    for await (const item of this) {
      items.push(item);
      if (items.length >= n) {
        break;
      }
    }
    return items;
  }

  /**
   * Find the first item matching a predicate
   */
  async find(predicate: (item: T) => boolean): Promise<T | undefined> {
    for await (const item of this) {
      if (predicate(item)) {
        return item;
      }
    }
    return undefined;
  }

  /**
   * Filter items matching a predicate
   */
  async *filter(predicate: (item: T) => boolean): AsyncGenerator<T> {
    for await (const item of this) {
      if (predicate(item)) {
        yield item;
      }
    }
  }

  /**
   * Map items using a transform function
   */
  async *map<U>(transform: (item: T) => U): AsyncGenerator<U> {
    for await (const item of this) {
      yield transform(item);
    }
  }
}

/**
 * Create a page iterator from a fetch function
 */
export function paginate<T>(
  fetchPage: (params: PaginationParams) => Promise<PaginatedResponse<T>>,
  params: PaginationParams = {}
): PageIterator<T> {
  return new PageIterator(fetchPage, params);
}
