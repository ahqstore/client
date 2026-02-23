use std::{sync::LazyLock, thread::sleep, time::Duration};

use win32_notif::{
  ManageNotification, NotificationActivatedEventHandler, NotificationBuilder, NotificationDataSet, ToastsNotifier, notification::{
    AdaptiveText, Scenario, actions::{ActionButton, action::ActivationType}, audio::{Audio, Src}, group::{Group, SubGroup}, visual::{image::{AdaptiveImageAlign, ImageCrop}, progress::ProgressValue, text::HintStyle, *}
  }
};

pub static NOTIF: LazyLock<ToastsNotifier> = LazyLock::new(|| {
  ToastsNotifier::new("Microsoft.Windows.Explorer").expect("Unexpected error while trying to create notifier")
});

pub(crate) fn send_update(total: u16) {
  let updated = format!("{total} apps will be updated");

  let notif = NotificationBuilder::new()
    .with_scenario(Scenario::Reminder)
    .audio(Audio::new(Src::Reminder, false, false))
    .visual(Text::create(0, "Updates Available").with_style(HintStyle::Title))
    .visual(
      Text::create(
        0,
        "Application updates are available, would you like to update?",
      )
      .with_style(HintStyle::Subtitle),
    )
    .visual(
      Group::new()
        .with_subgroup(
          SubGroup::new()
            .with_visual(
              Text::create(0, &updated)
                .with_style(HintStyle::Base)
            )
            .with_visual(
              Text::create(0, "Only unattended updates will be processed")
                .with_style(HintStyle::CaptionSubtle)
            )
        )
    )
    .action(ActionButton::create("Yes").with_id("update"))
    .action(ActionButton::create("No").with_id("no"))
    .on_activated(
      NotificationActivatedEventHandler::new(|_a, b| {
        let notif = b.unwrap();

        if &notif.button_id.unwrap_or_default() == "update" {
          panic!("Called update");
        }

        Ok(())
      })
    )
    .with_use_button_style(true)
    .build(0, &NOTIF, "2", "1")
    .unwrap();
    
  _ = notif.set_expires_on_reboot(true);

  notif.show()
    .unwrap();
}

pub(crate) fn app_id_updating(src: &str, app_id: &str, status: &str) {
  let notif = NotificationBuilder::new()
    .with_scenario(Scenario::Default)
    .audio(Audio::new(Src::IM, false, false))
    .visual(Text::create_binded(0, "title").with_style(HintStyle::Title))
    .visual(Text::create_binded(0, "body").with_style(HintStyle::Body))
    .visual(
      Image::create(1, src)
        .with_align(AdaptiveImageAlign::Default)
        .with_crop(ImageCrop::Circle)
        .with_placement(Placement::AppLogoOverride)
    )
    .visual(
      Progress::create(AdaptiveText::BindTo("txt"), ProgressValue::BindTo("prog"))
        .with_override_value(AdaptiveText::BindTo("txxt")),
    )
    .value("title", status)
    .value("body", "Please wait while we update it for you")
    .value("txt", "Downloading")
    .value("prog", "indeterminate")
    .value("txxt", "Starting up...")
    .with_use_button_style(true)
    .build(0, &NOTIF, app_id, "update")
    .unwrap();
    
  _ = notif.set_expires_on_reboot(true);

  notif.show()
    .unwrap();
}

pub(crate) fn updates_pending(apps: u16) {
  let pending = format!("{apps} apps have updates pending");

  let notif = NotificationBuilder::new()
    .with_scenario(Scenario::Reminder)
    .audio(Audio::new(Src::Reminder, false, false))
    .visual(Text::create(0, "Updates Pending").with_style(HintStyle::Title))
    .visual(
      Text::create(
        0,
        "A few updates are pending that require your attention.",
      )
      .with_style(HintStyle::Subtitle),
    )
    .visual(
      Group::new()
        .with_subgroup(
          SubGroup::new()
            .with_visual(
              Text::create(0, "{x}")
                .with_style(HintStyle::Base)
            )
            .with_visual(
              Text::create(0, "These require your action to update")
                .with_style(HintStyle::CaptionSubtle)
            )
        )
    )
    .value("x", &pending)
    .action(ActionButton::create("Launch store").with_activation_type(ActivationType::Protocol).with_id("launch"))
    .on_activated(
      NotificationActivatedEventHandler::new(|_a, b| {
        let notif = b.unwrap();

        println!("{notif:#?}");

        Ok(())
      })
    )
    .with_use_button_style(true)
    .build(0, &NOTIF, "2", "1")
    .unwrap();
    
  _ = notif.set_expires_on_reboot(true);

  notif.show()
    .unwrap();
}
