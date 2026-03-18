// src/services/nlp_service.rs
use crate::models::analysis::StructuredFinancialData;

/// Parses free-form financial text into a structured data object.
/// Uses heuristic pattern matching. For production, consider using
/// a dedicated NLP service or Gemini itself for extraction.
pub fn parse_financial_text(text: &str) -> StructuredFinancialData {
    let text_lower = text.to_lowercase();

    // ---- Income extraction ----
    let income = extract_income(&text_lower);

    // ---- Country detection ----
    let country = if text_lower.contains("india") || text_lower.contains("lakh") || text_lower.contains("rupee") || text_lower.contains("inr") {
        "India".to_string()
    } else if text_lower.contains("usa") || text_lower.contains("united states") || text_lower.contains("dollar") {
        "USA".to_string()
    } else if text_lower.contains("uk") || text_lower.contains("britain") || text_lower.contains("pound") {
        "UK".to_string()
    } else {
        "Unknown".to_string()
    };

    // ---- Family members ----
    let family_members = extract_family_members(&text_lower);

    // ---- Loans ----
    let mut loans = Vec::new();
    if text_lower.contains("home loan") || text_lower.contains("mortgage") { loans.push("home loan".to_string()); }
    if text_lower.contains("car loan") || text_lower.contains("auto loan") { loans.push("car loan".to_string()); }
    if text_lower.contains("personal loan") { loans.push("personal loan".to_string()); }
    if text_lower.contains("student loan") || text_lower.contains("education loan") { loans.push("student loan".to_string()); }
    if text_lower.contains("credit card debt") || text_lower.contains("credit card") { loans.push("credit card debt".to_string()); }

    // ---- Financial challenges ----
    let mut challenges = Vec::new();
    if text_lower.contains("low saving") || text_lower.contains("struggle with saving") || text_lower.contains("can't save") {
        challenges.push("low savings".to_string());
    }
    if text_lower.contains("high expense") || text_lower.contains("spending too much") {
        challenges.push("high expenses".to_string());
    }
    if text_lower.contains("debt") { challenges.push("debt burden".to_string()); }
    if text_lower.contains("no investment") || text_lower.contains("not investing") {
        challenges.push("no investments".to_string());
    }
    if challenges.is_empty() {
        challenges.push("general financial planning".to_string());
    }

    // ---- Goals ----
    let mut goals = Vec::new();
    if text_lower.contains("reduce tax") || text_lower.contains("save tax") || text_lower.contains("tax saving") {
        goals.push("reduce tax".to_string());
    }
    if text_lower.contains("build wealth") || text_lower.contains("wealth creation") {
        goals.push("build wealth".to_string());
    }
    if text_lower.contains("retire") || text_lower.contains("retirement") {
        goals.push("retirement planning".to_string());
    }
    if text_lower.contains("buy house") || text_lower.contains("own home") {
        goals.push("buy a house".to_string());
    }
    if text_lower.contains("emergency fund") {
        goals.push("build emergency fund".to_string());
    }
    if goals.is_empty() {
        goals.push("improve financial health".to_string());
    }

    StructuredFinancialData {
        income,
        country,
        family_members,
        loans,
        financial_challenges: challenges,
        goals,
    }
}

/// Extracts income from text (supports "lakh", "thousand", plain numbers)
fn extract_income(text: &str) -> f64 {
    // Match patterns like "8 lakh", "800000", "80,000"
    let re_lakh = regex::Regex::new(r"(\d+(?:\.\d+)?)\s*lakh").unwrap();
    let re_thousand = regex::Regex::new(r"(\d+(?:\.\d+)?)\s*thousand").unwrap();
    let re_plain = regex::Regex::new(r"\b(\d{4,10})\b").unwrap();

    if let Some(cap) = re_lakh.captures(text) {
        let val: f64 = cap[1].parse().unwrap_or(0.0);
        return val * 100_000.0;
    }
    if let Some(cap) = re_thousand.captures(text) {
        let val: f64 = cap[1].parse().unwrap_or(0.0);
        return val * 1_000.0;
    }
    if let Some(cap) = re_plain.captures(text) {
        return cap[1].replace(",", "").parse().unwrap_or(0.0);
    }
    0.0
}

/// Extracts approximate number of family members from text
fn extract_family_members(text: &str) -> u32 {
    let re = regex::Regex::new(r"(\d+)\s*(?:kids?|children|child|members?|family of)").unwrap();
    if let Some(cap) = re.captures(text) {
        let kids: u32 = cap[1].parse().unwrap_or(0);
        return kids + 2; // Assume parents
    }
    if text.contains("married") { return 2; }
    if text.contains("single") || text.contains("bachelor") { return 1; }
    2 // Default
}