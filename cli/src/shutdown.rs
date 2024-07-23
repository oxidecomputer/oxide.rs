use crate::eprintln_nopipe;

use anyhow::{anyhow, Error, Result};
use std::future::Future;
use tokio::signal;

/// GracefulShutdown listens for CTRL+C events and excutes futures with
/// a cleanup action on failure or interrupt. A second CTRL+C will cause the
/// process to exit immediately.
#[derive(Debug)]
pub struct GracefulShutdown {
    /// The channel which will provide notification that a CTRL+C signal
    /// has been received.
    rx: tokio::sync::watch::Receiver<()>,
}

impl GracefulShutdown {
    /// Construct a new GracefulShutdown and start listening for CTRL+C events.
    pub fn new(cancel_msg: &'static str, exit_msg: &'static str) -> Self {
        let (tx, rx) = tokio::sync::watch::channel(());

        tokio::spawn(async move {
            let mut force_shutdown = false;
            loop {
                signal::ctrl_c().await.expect("Failed to listen for CTRL+C");

                if force_shutdown {
                    eprintln_nopipe!("Shutting down immediately.\n{exit_msg}.");
                    std::process::exit(130);
                }
                force_shutdown = true;

                eprintln_nopipe!("{cancel_msg}. Press CTRL+C again to exit immediately.");
                tx.send(()).expect("Failed to write to shutdown channel");
            }
        });

        Self { rx }
    }

    /// Execute the provided `Future` while listening for a CTRL+C event.
    /// If the `Future` fails, or CTRL+C is received, run the cleanup task.
    /// If cleanup is executed, then an error will be returned even if it
    /// completes successfully. The `Future` will not be executed if a
    /// CTRL+C event has already been received.
    pub async fn run_with_cleanup<F, T, C, E>(
        &mut self,
        future: F,
        cleanup: Cleanup<C>,
    ) -> Result<T>
    where
        F: Future<Output = Result<T, E>>,
        E: Into<anyhow::Error>,
        C: Future<Output = Result<()>>,
    {
        let cancel_err = anyhow!("user canceled request");

        if self.rx.has_changed()? {
            return Err(cleanup.run(cancel_err).await);
        }

        tokio::select! {
            result = future => {
                match result {
                    Ok(o) => Ok(o),
                    Err(e) => {
                        Err(cleanup.run(e.into()).await)
                    }
                }
            },
            _ = self.rx.changed() => {
                Err(cleanup.run(cancel_err).await)
            }
        }
    }

    /// Whether the caller should start to unwind the command being executed.
    pub fn shutdown_requested(&self) -> Result<bool> {
        self.rx.has_changed().map_err(|e| e.into())
    }
}

/// A `Future` to be run on interrupt or failure.
#[derive(Debug)]
pub struct Cleanup<C>
where
    C: Future<Output = Result<()>>,
{
    pub future: C,
    pub context: &'static str,
}

impl<C> Cleanup<C>
where
    C: Future<Output = Result<()>>,
{
    async fn run(self, source_err: Error) -> Error {
        match self.future.await {
            Ok(()) => source_err.context(self.context),
            Err(e) => source_err.context(e).context(self.context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Error;

    #[tokio::test]
    async fn test_shutdown_future_ok() {
        let mut gs = GracefulShutdown::new("foo", "bar");

        let result: Result<(), Error> = gs
            .run_with_cleanup::<_, (), _, Error>(
                async { Ok(()) },
                Cleanup {
                    future: async { Err(anyhow!("ran cleanup unexpectedly")) },
                    context: "bar",
                },
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_shutdown_future_failed_cleanup_ok() {
        let mut gs = GracefulShutdown::new("foo", "bar");

        let result: Result<(), Error> = gs
            .run_with_cleanup(
                async { Err(anyhow!("future failed")) },
                Cleanup {
                    future: async { Ok(()) },
                    context: "failed to frobnicate",
                },
            )
            .await;

        let err = match result {
            Ok(_) => panic!("unexpected OK"),
            Err(e) => format!("{e:#}"),
        };
        assert_eq!("failed to frobnicate: future failed", err);
    }

    #[tokio::test]
    async fn test_shutdown_future_failed_cleanup_failed() {
        let mut gs = GracefulShutdown::new("foo", "bar");

        let result: Result<(), Error> = gs
            .run_with_cleanup(
                async { Err(anyhow!("future failed")) },
                Cleanup {
                    future: async { Err(anyhow!("cleanup failed")) },
                    context: "failed to frobnicate",
                },
            )
            .await;

        let err = match result {
            Ok(_) => panic!("unexpected OK"),
            Err(e) => format!("{e:#}"),
        };
        assert_eq!("failed to frobnicate: cleanup failed: future failed", err);
    }

    #[tokio::test]
    async fn test_shutdown_cancel_cleanup_ok() {
        let mut gs = GracefulShutdown::new("foo", "bar");
        gs.rx.mark_changed();

        let result: Result<(), Error> = gs
            .run_with_cleanup::<_, (), _, Error>(
                async { Ok(()) },
                Cleanup {
                    future: async { Ok(()) },
                    context: "failed to frobnicate",
                },
            )
            .await;

        let err = match result {
            Ok(_) => panic!("unexpected OK"),
            Err(e) => format!("{e:#}"),
        };
        assert_eq!("failed to frobnicate: user canceled request", err);
    }

    #[tokio::test]
    async fn test_shutdown_cancel_cleanup_failed() {
        let mut gs = GracefulShutdown::new("foo", "bar");
        gs.rx.mark_changed();

        let result: Result<(), Error> = gs
            .run_with_cleanup::<_, (), _, Error>(
                async { Ok(()) },
                Cleanup {
                    future: async { Err(anyhow!("cleanup failed")) },
                    context: "failed to frobnicate",
                },
            )
            .await;

        let err = match result {
            Ok(_) => panic!("unexpected OK"),
            Err(e) => format!("{e:#}"),
        };
        assert_eq!(
            "failed to frobnicate: cleanup failed: user canceled request",
            err
        );
    }

    #[tokio::test]
    async fn test_shutdown_requested() {
        let mut gs = GracefulShutdown::new("foo", "bar");

        assert!(!gs.shutdown_requested().unwrap());

        gs.rx.mark_changed();

        assert!(gs.shutdown_requested().unwrap());
    }
}
