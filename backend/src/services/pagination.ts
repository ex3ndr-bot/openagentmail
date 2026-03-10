import { config } from '../config.js';

interface PaginationOptions {
  limit?: number;
  pageToken?: string;
}

interface CursorData {
  lastId: string;
  createdAt?: string;
}

export function encodeCursor(data: CursorData): string {
  return Buffer.from(JSON.stringify(data)).toString('base64url');
}

export function decodeCursor(token: string): CursorData | null {
  try {
    const json = Buffer.from(token, 'base64url').toString('utf-8');
    return JSON.parse(json);
  } catch {
    return null;
  }
}

export function getPaginationParams(options: PaginationOptions) {
  const limit = Math.min(
    options.limit || config.pagination.defaultLimit,
    config.pagination.maxLimit
  );

  return {
    limit: limit + 1, // Fetch one extra to check if there are more
    cursor: options.pageToken ? decodeCursor(options.pageToken) : null,
  };
}

export function formatPaginatedResponse<T extends { id: string; createdAt?: Date }>(
  items: T[],
  requestedLimit: number,
  formatItem: (item: T) => unknown
) {
  const limit = Math.min(requestedLimit || config.pagination.defaultLimit, config.pagination.maxLimit);
  const hasMore = items.length > limit;
  const resultItems = items.slice(0, limit);

  let nextPageToken: string | null = null;
  if (hasMore && resultItems.length > 0) {
    const lastItem = resultItems[resultItems.length - 1];
    nextPageToken = encodeCursor({
      lastId: lastItem.id,
      createdAt: lastItem.createdAt?.toISOString(),
    });
  }

  return {
    items: resultItems.map(formatItem),
    next_page_token: nextPageToken,
    has_more: hasMore,
  };
}
