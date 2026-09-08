use brevo_v3::types::*;
use std::str::FromStr;

#[test]
fn test_email_parsing_valid() {
    let email = Email::new("user@example.com").expect("valid email");
    assert_eq!(email.as_str(), "user@example.com");
    assert_eq!(&*email, "user@example.com");
    assert_eq!(email.to_string(), "user@example.com");

    let parsed = Email::from_str("admin@sub.domain.org").expect("valid parsed");
    assert_eq!(parsed.as_str(), "admin@sub.domain.org");
}

#[test]
fn test_email_parsing_invalid() {
    assert!(Email::new("").is_err());
    assert!(Email::new("   ").is_err());
    assert!(Email::new("no-at-sign").is_err());
}

#[test]
fn test_email_serde() {
    let email = Email::new("test@domain.com").unwrap();
    let serialized = serde_json::to_string(&email).unwrap();
    assert_eq!(serialized, "\"test@domain.com\"");

    let deserialized: Email = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, email);
}

#[test]
fn test_phone_number_invariants() {
    let phone = PhoneNumber::new("+12025550123").expect("valid phone");
    assert_eq!(phone.as_str(), "+12025550123");
    assert_eq!(&*phone, "+12025550123");

    assert!(PhoneNumber::new("").is_err());
    assert!(PhoneNumber::new("   ").is_err());
}

#[test]
fn test_numeric_id_newtypes() {
    let contact_id = ContactId::new(100);
    assert_eq!(contact_id.get(), 100);
    assert_eq!(i64::from(contact_id), 100);
    assert_eq!(contact_id.to_string(), "100");

    let campaign_id = CampaignId::from(200);
    assert_eq!(campaign_id.get(), 200);

    let template_id = TemplateId::from(300);
    assert_eq!(template_id.get(), 300);

    let webhook_id = WebhookId::from(400);
    assert_eq!(webhook_id.get(), 400);

    let process_id = ProcessId::from(500);
    assert_eq!(process_id.get(), 500);
}

#[test]
fn test_string_id_newtypes() {
    let deal_id = DealId::new("deal_abc_123");
    assert_eq!(deal_id.as_str(), "deal_abc_123");
    assert_eq!(&*deal_id, "deal_abc_123");
    assert_eq!(deal_id.to_string(), "deal_abc_123");

    let company_id = CompanyId::from("comp_456");
    assert_eq!(company_id.as_str(), "comp_456");

    let task_id = TaskId::from("task_789");
    assert_eq!(task_id.as_str(), "task_789");

    let note_id = NoteId::from("note_001");
    assert_eq!(note_id.as_str(), "note_001");

    let pipeline_id = PipelineId::from("pipe_main");
    assert_eq!(pipeline_id.as_str(), "pipe_main");
}
