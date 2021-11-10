use error_chain::error_chain;

error_chain! {
    links {
        Core(::falcon_core::errors::Error, ::falcon_core::errors::ErrorKind);
    }
}