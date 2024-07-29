#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Config {
    log: bool,
    notify: bool,
}

impl Config {
    pub fn log(mut self) -> Self {
        self.log = !self.log;
        self
    }

    pub fn notify(mut self) -> Self {
        self.notify = !self.notify;
        self
    }
}

#[derive(Default)]
pub struct Observer {
    config: Config,
    notify: egui_notify::Toasts,
}

impl Observer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_config(config: Config) -> Self {
        let mut obs = Self::new();
        obs.config = config;
        obs
    }

    pub fn trace(&mut self, msg: &str) {
        if self.config.log {
            tracing::trace!(msg);
        }
        if self.config.notify {
            self.notify.basic(msg);
        }
    }

    pub fn info(&mut self, msg: &str) {
        if self.config.log {
            tracing::info!(msg);
        }
        if self.config.notify {
            self.notify.info(msg);
        }
    }

    pub fn warn(&mut self, msg: &str) {
        if self.config.log {
            tracing::warn!(msg);
        }
        if self.config.notify {
            self.notify.warning(msg);
        }
    }

    pub fn success(&mut self, msg: &str) {
        if self.config.log {
            tracing::info!(msg);
        }
        if self.config.notify {
            self.notify.success(msg);
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        self.notify.show(ctx);
    }
}
