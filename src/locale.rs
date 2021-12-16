// TODO move global locale stuff in its own thing
fluent_templates::static_loader! {
    pub static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
        // core_locales: "./locales/core.ftl",
    };
}
