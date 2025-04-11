use crate::{error::*, notification::Notification};

pub use mac_notification_sys::error::{ApplicationError, Error as MacOsError, NotificationError};

use std::ops::{Deref, DerefMut};

/// A handle to a shown notification.
///
/// This keeps a connection alive to ensure actions work on certain desktops.
#[derive(Debug)]
pub struct NotificationHandle {
    notification: Notification,
}

impl NotificationHandle {
    #[allow(missing_docs)]
    pub fn new(notification: Notification) -> NotificationHandle {
        NotificationHandle { notification }
    }
}

impl Deref for NotificationHandle {
    type Target = Notification;

    fn deref(&self) -> &Notification {
        &self.notification
    }
}

/// Allow to easily modify notification properties
impl DerefMut for NotificationHandle {
    fn deref_mut(&mut self) -> &mut Notification {
        &mut self.notification
    }
}

pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    let noti = mac_notification_sys::Notification::default()
        .title(notification.summary.as_str())
        .message(&notification.body)
        .maybe_subtitle(notification.subtitle.as_deref())
        .maybe_sound(notification.sound_name.as_deref());

    if !notification.actions.is_empty() {
        let action = notification.actions.first().cloned();
        let action = action.unwrap_or("Click".to_string());
        let response = noti
            .main_button(mac_notification_sys::MainButton::SingleAction(action))
            .send()?;

        match response {
            mac_notification_sys::NotificationResponse::None => {}
            mac_notification_sys::NotificationResponse::ActionButton(_) => todo!(),
            mac_notification_sys::NotificationResponse::CloseButton(_) => todo!(),
            mac_notification_sys::NotificationResponse::Click => todo!(),
            mac_notification_sys::NotificationResponse::Reply(_) => todo!(),
        }
    }

    Ok(NotificationHandle::new(notification.clone()))
}

pub(crate) fn schedule_notification(
    notification: &Notification,
    delivery_date: f64,
) -> Result<NotificationHandle> {
    mac_notification_sys::Notification::default()
        .title(notification.summary.as_str())
        .message(&notification.body)
        .maybe_subtitle(notification.subtitle.as_deref())
        .maybe_sound(notification.sound_name.as_deref())
        .delivery_date(delivery_date)
        .send()?;

    Ok(NotificationHandle::new(notification.clone()))
}
