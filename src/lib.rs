pub use aquila_core::*;

#[cfg(feature = "server")]
pub mod server {
    pub use aquila_server::*;
}

#[cfg(feature = "client")]
pub mod client {
    pub use aquila_client::*;
}

#[cfg(feature = "fs")]
pub mod fs {
    pub use aquila_fs::*;
}

#[cfg(feature = "mock_auth")]
pub mod auth_mock {
    pub use aquila_auth_mock::*;
}

#[cfg(feature = "s3")]
pub mod s3 {
    pub use aquila_s3::*;
}

#[cfg(feature = "opendal")]
pub mod opendal {
    pub use aquila_opendal::*;
}

#[cfg(feature = "github_auth")]
pub mod auth_github {
    pub use aquila_auth_github::*;
}

pub mod prelude {
    pub use aquila_core::prelude::*;

    #[cfg(feature = "server")]
    pub use aquila_server::prelude::*;

    #[cfg(feature = "client")]
    pub use aquila_client::AquilaClient;

    #[cfg(feature = "fs")]
    pub use aquila_fs::FileSystemStorage;

    #[cfg(feature = "mock_auth")]
    pub use aquila_auth_mock::AllowAllAuth;

    #[cfg(feature = "github_auth")]
    pub use aquila_auth_github::{GithubAuthProvider, GithubConfig};

    #[cfg(feature = "s3")]
    pub use aquila_s3::S3Storage;

    #[cfg(feature = "opendal")]
    pub use aquila_opendal::OpendalStorage;
}
