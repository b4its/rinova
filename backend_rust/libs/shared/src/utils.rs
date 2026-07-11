use validator::Validate;

pub fn validate_email(email: &str) -> bool {
    let email_regex = regex::Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
    email_regex.is_match(email)
}

pub fn validate_password(password: &str) -> bool {
    // Minimum 8 characters
    if password.len() < 8 {
        return false;
    }

    // Contains uppercase
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    
    // Contains lowercase
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    
    // Contains digit
    let has_digit = password.chars().any(|c| c.is_numeric());

    has_uppercase && has_lowercase && has_digit
}

pub fn sanitize_input(input: &str) -> String {
    // Remove potential SQL injection patterns
    let sanitized = input
        .replace("'", "")
        .replace("\"", "")
        .replace(";", "")
        .replace("--", "")
        .replace("/*", "")
        .replace("*/", "");

    // Remove script tags
    let sanitized = regex::Regex::new(r"<script[^>]*>.*?</script>")
        .unwrap()
        .replace_all(&sanitized, "");

    sanitized.to_string()
}

pub fn is_valid_domain(domain: &str) -> bool {
    // Domain length 1-253 characters
    if domain.is_empty() || domain.len() > 253 {
        return false;
    }

    // Valid domain format
    let domain_regex = regex::Regex::new(
        r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)*[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?$"
    ).unwrap();

    domain_regex.is_match(domain)
}
