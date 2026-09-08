use brevo_v3::types::*;
use serde_json::json;

#[test]
fn test_serde_newtypes_roundtrip() {
    let email = Email::new("finance@company.com").unwrap();
    let contact_id = ContactId::new(42);
    let campaign_id = CampaignId::new(101);
    let deal_id = DealId::new("deal_999");
    let domain = DomainName::new("example.com");

    let payload = json!({
        "email": email,
        "contact_id": contact_id,
        "campaign_id": campaign_id,
        "deal_id": deal_id,
        "domain": domain,
    });

    let s = serde_json::to_string(&payload).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&s).unwrap();

    assert_eq!(parsed["email"], "finance@company.com");
    assert_eq!(parsed["contact_id"], 42);
    assert_eq!(parsed["campaign_id"], 101);
    assert_eq!(parsed["deal_id"], "deal_999");
    assert_eq!(parsed["domain"], "example.com");
}
