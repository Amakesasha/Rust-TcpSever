use crate::*;

/// Enumeration for displaying information about the server's operation.
pub enum TypeServer {
    /// HTTP server.
    Http(usize),
    /// Clean server.
    Clean(usize),
}

/// Functions for notifying about server operation.
impl TypeServer {
    #[inline]
    /// Conclusion about server startup.
    /// * listener = TcpListener.
    /// * type_server = Server type.
    pub fn launch(listener: &TcpListener, type_server: TypeServer) {
        match type_server {
            TypeServer::Http(number) => println!(
                "SERVER | HTTP | {} | {number} | LAUNCH",
                listener.local_addr().unwrap()
            ),
            TypeServer::Clean(number) => println!(
                "SERVER | CLEAN | {} | {number} | LAUNCH",
                listener.local_addr().unwrap()
            ),
        }
    }

    #[inline]
    /// Conclusion about server shutdown.
    /// * listener = TcpListener.
    /// * type_server = Server type.
    pub fn shotdown(listener: &TcpListener, type_server: TypeServer) {
        match type_server {
            TypeServer::Http(number) => println!(
                "SERVER | HTTP | {} | {number} | SHOT DOWN",
                listener.local_addr().unwrap()
            ),
            TypeServer::Clean(number) => println!(
                "SERVER | CLEAN | {} | {number} | SHOT DOWN",
                listener.local_addr().unwrap()
            ),
        }
    }
}
