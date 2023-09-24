use std::sync::Arc;

use druid::{
  lens::{self, LensExt},
  widget::{Button, Flex, Label, List},
  AppLauncher, Data, Env, Lens, PlatformError, Widget, WidgetExt, WindowDesc,
};

use walleth::{Controller, Keychain};

#[derive(Clone, Lens)]
pub struct AppState {
  keychain: Keychain,
}

impl AppState {
  pub fn new() -> Self {
    Self {
      keychain: Keychain::new(),
    }
  }
}

impl Data for AppState {
  fn same(&self, other: &Self) -> bool {
    self.keychain == other.keychain
  }
}

fn main() -> Result<(), PlatformError> {
  let main_window = WindowDesc::new(ui::render());

  let state = AppState::new();

  AppLauncher::with_window(main_window)
    .log_to_console()
    .launch(state)
}

mod ui {
  use super::*;

  pub fn render() -> impl Widget<AppState> {
    let accounts_list = accounts_list().lens(lens::Identity.map(
      |data: &AppState| {
        Arc::new(
          data
            .keychain
            .get_state()
            .accounts
            .iter()
            .map(|account| account.address.clone())
            .collect::<Vec<String>>(),
        )
      },
      |_, _| {},
    ));

    let add_account_button =
      button_with_label("Add account").on_click(|_, data: &mut AppState, _| {
        data.keychain.add_account().unwrap();
      });

    let lock_wallet_button =
      button_with_label("Lock wallet").on_click(|_, data: &mut AppState, _| {
        data.keychain.lock("password").unwrap();
      });

    Flex::column()
      .with_child(lock_wallet_button)
      .with_child(add_account_button)
      .with_child(accounts_list)
  }

  fn accounts_list() -> Flex<Arc<Vec<String>>> {
    Flex::row().with_child(List::new(|| {
      Label::new(|account: &String, _: &Env| format!("Account: {}", account))
        .padding(5.0)
        .center()
    }))
  }

  fn button_with_label(label: &str) -> Button<AppState> {
    Button::new(label)
  }
}
