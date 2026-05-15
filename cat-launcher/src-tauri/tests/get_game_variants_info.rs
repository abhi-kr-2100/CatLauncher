mod support;

use cat_launcher_lib::variants::GameVariant;
use cat_launcher_lib::variants::get_game_variants_info::get_game_variants_info;
use cat_launcher_lib::variants::repository::game_variant_order_repository::GameVariantOrderRepository;
use cat_launcher_lib::variants::repository::sqlite_game_variant_order_repository::SqliteGameVariantOrderRepository;

use support::db::TestDatabase;

#[tokio::test]
async fn returns_all_variants_in_enum_order_when_no_order_is_saved() {
  let db = TestDatabase::new();
  let repository =
    SqliteGameVariantOrderRepository::new(db.pool.clone());

  let variants = get_game_variants_info(&repository)
    .await
    .expect("get variants");

  let ids = variants
    .iter()
    .map(|variant| variant.id)
    .collect::<Vec<_>>();
  let names = variants
    .iter()
    .map(|variant| variant.name.as_str())
    .collect::<Vec<_>>();

  assert_eq!(
    ids,
    vec![
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ]
  );
  assert_eq!(
    names,
    vec!["Dark Days Ahead", "Bright Nights", "The Last Generation"]
  );
}

#[tokio::test]
async fn returns_variants_in_saved_order() {
  let db = TestDatabase::new();
  let repository =
    SqliteGameVariantOrderRepository::new(db.pool.clone());
  repository
    .update_order(&[
      GameVariant::TheLastGeneration,
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
    ])
    .await
    .expect("save variant order");

  let variants = get_game_variants_info(&repository)
    .await
    .expect("get variants");

  let ids = variants
    .iter()
    .map(|variant| variant.id)
    .collect::<Vec<_>>();
  let names = variants
    .iter()
    .map(|variant| variant.name.as_str())
    .collect::<Vec<_>>();

  assert_eq!(
    ids,
    vec![
      GameVariant::TheLastGeneration,
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
    ]
  );
  assert_eq!(
    names,
    vec!["The Last Generation", "Dark Days Ahead", "Bright Nights"]
  );
}
