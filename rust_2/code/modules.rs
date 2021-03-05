mod server {
    pub mod backend {
        pub fn fix_backend() {}
    }
}

pub fn fix_site() {
    // Absolute path
    crate::server::backend::fix_backend();

    // Relative path
    server::backend::fix_backend();
}
