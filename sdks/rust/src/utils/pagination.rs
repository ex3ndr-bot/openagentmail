//! Pagination utilities for streaming results

use futures::Stream;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::error::Result;
use crate::types::PaginatedResponse;

/// A stream that handles cursor-based pagination automatically
pub struct PaginatedStream<T, F, Fut>
where
    F: Fn(Option<String>) -> Fut,
    Fut: Future<Output = Result<PaginatedResponse<T>>>,
{
    fetch_fn: F,
    current_items: Vec<T>,
    next_page_token: Option<String>,
    has_more: bool,
    is_first_fetch: bool,
    pending_future: Option<Pin<Box<Fut>>>,
}

impl<T, F, Fut> PaginatedStream<T, F, Fut>
where
    F: Fn(Option<String>) -> Fut,
    Fut: Future<Output = Result<PaginatedResponse<T>>>,
{
    /// Create a new paginated stream
    pub fn new(fetch_fn: F) -> Self {
        Self {
            fetch_fn,
            current_items: Vec::new(),
            next_page_token: None,
            has_more: true,
            is_first_fetch: true,
            pending_future: None,
        }
    }
}

impl<T, F, Fut> Stream for PaginatedStream<T, F, Fut>
where
    T: Unpin,
    F: Fn(Option<String>) -> Fut + Unpin,
    Fut: Future<Output = Result<PaginatedResponse<T>>> + Unpin,
{
    type Item = Result<T>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = &mut *self;

        // If we have items buffered, return them one by one
        if !this.current_items.is_empty() {
            return Poll::Ready(Some(Ok(this.current_items.remove(0))));
        }

        // If no more pages and no buffered items, we're done
        if !this.has_more && !this.is_first_fetch {
            return Poll::Ready(None);
        }

        // Need to fetch the next page
        if this.pending_future.is_none() {
            let token = this.next_page_token.take();
            let future = (this.fetch_fn)(token);
            this.pending_future = Some(Box::pin(future));
        }

        // Poll the pending future
        if let Some(ref mut future) = this.pending_future {
            match Pin::new(future).poll(cx) {
                Poll::Ready(result) => {
                    this.pending_future = None;
                    this.is_first_fetch = false;

                    match result {
                        Ok(response) => {
                            this.current_items = response.items;
                            this.next_page_token = response.next_page_token;
                            this.has_more = response.has_more;

                            // Return the first item if available
                            if !this.current_items.is_empty() {
                                Poll::Ready(Some(Ok(this.current_items.remove(0))))
                            } else if this.has_more {
                                // Empty page but more available, try again
                                cx.waker().wake_by_ref();
                                Poll::Pending
                            } else {
                                Poll::Ready(None)
                            }
                        }
                        Err(e) => Poll::Ready(Some(Err(e))),
                    }
                }
                Poll::Pending => Poll::Pending,
            }
        } else {
            Poll::Pending
        }
    }
}

/// Helper trait for converting paginated responses into streams
pub trait IntoPaginatedStream<T> {
    /// Convert into a stream that automatically handles pagination
    fn into_stream<F, Fut>(self, fetch_fn: F) -> PaginatedStream<T, F, Fut>
    where
        F: Fn(Option<String>) -> Fut,
        Fut: Future<Output = Result<PaginatedResponse<T>>>;
}
