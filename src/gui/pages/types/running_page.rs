use crate::translations::translations::{notifications_translation, overview_translation};
use crate::Language;

/// This enum defines the current running page.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RunningPage {
    /// Overview page.
    Overview,
    // /// Inspect page.
    // Inspect,
    /// Notifications page.
    Notifications,
}

impl RunningPage {
    pub fn get_tab_label(&self, language: Language) -> &str {
        match self {
            RunningPage::Overview => overview_translation(language),
            // RunningPage::Inspect => inspect_translation(language),
            RunningPage::Notifications => notifications_translation(language),
        }
    }

    pub fn next(self) -> Self {
        match self {
            RunningPage::Overview => RunningPage::Notifications,
            RunningPage::Notifications => RunningPage::Overview,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            RunningPage::Overview => RunningPage::Notifications,
            RunningPage::Notifications => RunningPage::Overview,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RunningPage;

    #[test]
    fn test_previous_running_page() {
        assert_eq!(RunningPage::Overview.previous(), RunningPage::Notifications);
        assert_eq!(RunningPage::Notifications.previous(), RunningPage::Overview);
    }

    #[test]
    fn test_next_running_page() {
        assert_eq!(RunningPage::Overview.next(), RunningPage::Notifications);
        assert_eq!(RunningPage::Notifications.next(), RunningPage::Overview);
    }
}
