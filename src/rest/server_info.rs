use crate::*;

/// Enumeration for displaying information about the server's operation.
pub enum ServerInfo {
    /// HTTP server.
    Http,
    /// Clean server.
    Clean,
}

/// Functions for notifying about server operation
impl ServerInfo {
    #[inline]
    /// Conclusion about server startup.
    /// * listener = TcpListener.
    /// * type_server = Server type.
    pub fn launch(listener: &TcpListener, type_server: ServerInfo) {
        match type_server {
            ServerInfo::Http => println!(
                "SERVER | HTTP | {} | LAUNCH",
                listener.local_addr().unwrap()
            ),
            ServerInfo::Clean => println!(
                "SERVER | CLEAN | {} | LAUNCH",
                listener.local_addr().unwrap()
            )
        }
    }

    #[inline]
    /// Conclusion about server shutdown.
    /// * listener = TcpListener.
    /// * type_server = Server type.
    pub fn shotdown(listener: &TcpListener, type_server: ServerInfo) {
        match type_server {
            ServerInfo::Http => println!(
                "SERVER | HTTP | {} | SHOT DOWN",
                listener.local_addr().unwrap()
            ),
            ServerInfo::Clean => println!(
                "SERVER | CLEAN | {} | SHOT DOWN",
                listener.local_addr().unwrap()
            ),
        }
    }
}
