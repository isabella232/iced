//! Configure your application.
use crate::window;
use crate::Font;

/// The settings of an application.
#[derive(Debug, Clone)]
pub struct Settings<Flags> {
    /// The identifier of the application.
    ///
    /// If provided, this identifier may be used to identify the application or
    /// communicate with it through the windowing system.
    pub id: Option<String>,

    /// The window settings.
    ///
    /// They will be ignored on the Web.
    pub window: window::Settings,

    /// The data needed to initialize the [`Application`].
    ///
    /// [`Application`]: crate::Application
    pub flags: Flags,

    /// The default [`Font`] to be used.
    ///
    /// By default, it uses [`Font::SansSerif`].
    pub default_font: Font,

    /// The text size that will be used by default.
    ///
    /// The default value is `16.0`.
    pub default_text_size: f32,

    /// If set to true, the renderer will try to perform antialiasing for some
    /// primitives.
    ///
    /// Enabling it can produce a smoother result in some widgets, like the
    /// [`Canvas`], at a performance cost.
    ///
    /// By default, it is disabled.
    ///
    /// [`Canvas`]: crate::widget::Canvas
    pub antialiasing: bool,

    /// Whether the [`Application`] should exit when the user requests the
    /// window to close (e.g. the user presses the close button).
    ///
    /// By default, it is enabled.
    ///
    /// [`Application`]: crate::Application
    pub exit_on_close_request: bool,
}

impl<Flags> Settings<Flags> {
    /// Initialize [`Application`] settings using the given data.
    ///
    /// [`Application`]: crate::Application
    pub fn with_flags(flags: Flags) -> Self {
        let default_settings = Settings::<()>::default();

        Self {
            flags,
            id: default_settings.id,
            window: default_settings.window,
            default_font: default_settings.default_font,
            default_text_size: default_settings.default_text_size,
            antialiasing: default_settings.antialiasing,
            exit_on_close_request: default_settings.exit_on_close_request,
        }
    }
}

impl<Flags> Default for Settings<Flags>
where
    Flags: Default,
{
    fn default() -> Self {
        Self {
            id: None,
            window: Default::default(),
            flags: Default::default(),
            default_font: Default::default(),
            default_text_size: 16.0,
            antialiasing: false,
            exit_on_close_request: true,
        }
    }
}

impl<Flags> From<Settings<Flags>> for iced_winit::Settings<Flags> {
    fn from(settings: Settings<Flags>) -> iced_winit::Settings<Flags> {
        iced_winit::Settings {
            id: settings.id,
            window: settings.window.into(),
            flags: settings.flags,
            exit_on_close_request: settings.exit_on_close_request,
        }
    }
}
