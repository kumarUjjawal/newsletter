pub mod configuration;
pub mod routes;
pub mod startup;

// Re-export the application entrypoint from `startup`
pub use startup::run;
