#[test]
fn test_create_success() -> Result<(), ProgramError> {
  // 1. Create a mock context with required accounts
  let mut context = MockContext::default();
  let user_key = context.payer.key();
  let campaign_key = Pubkey::new(&[1u8; 32]); // Replace with a unique key

  // 2. Set up campaign data
  let name = "Test Campaign".to_string();
  let description = "This is a test campaign for demonstration purposes.".to_string();
  let target_amount = 1000000000; // 1 SOL
  let project_url = "https://example.com/project".to_string();
  let progress_update_url = "https://example.com/updates".to_string();
  let project_image_url = "https://example.com/image.jpg".to_string();
  let category = "Technology".to_string();

  // 3. Create accounts with mock data
  let mut campaign_account = Account::new_empty(9000);
  context.accounts.campaign = campaign_account.to_account_info().clone();
  context.accounts.user = context.payer.to_account_info().clone();

  // 4. Call the create instruction
  smart_contracts::create(context.clone(), name, description, target_amount, project_url, progress_update_url, project_image_url, category)?;

  // 5. Assert campaign data is set correctly
  let campaign_data = &context.accounts.campaign.data.borrow()[..];
  let campaign: Campaign = Account::<Campaign>::try_deserialize(&mut &campaign_data[..])?;
  assert_eq!(campaign.name, name);
  assert_eq!(campaign.description, description);
  assert_eq!(campaign.target_amount, target_amount);
  assert_eq!(campaign.project_url, project_url);
  assert_eq!(campaign.progress_update_url, progress_update_url);
  assert_eq!(campaign.project_image_url, project_image_url);
  assert_eq!(campaign.category, category);
  assert_eq!(campaign.admin, user_key);
  assert_eq!(campaign.amount_donated, 0);
  assert_eq!(campaign.amount_withdrawn, 0);

  Ok(())
}



