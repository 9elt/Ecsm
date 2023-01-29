use crate::utils::config::ECSMConfig;

pub fn html_template(config: &ECSMConfig) -> String {
    format!(
        "\
<!DOCTYPE html>
<html lang=\"en\">

<head>
    <meta charset=\"UTF-8\">
    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>{}</title>
    <link rel=\"stylesheet\" href=\"css/main.css\">
</head>

<body>

<!-- 

    ECSM | [{}] project

    boolean state usage:

    <h1 handle_state=\"state_name\">click here</h1>

    option state usage:

    <h1 handle_state=\"otp_state_name:key_name\">click here</h1>
    <h1 handle_state=\"otp_state_name:second_key_name\">click here</h1>

-->

</body>

</html>
",
        config.name(),
        config.name()
    )
}

pub fn css_template(config: &ECSMConfig) -> String {
    format!(
        "\
/*

ECSM | [{}] project

boolean states usage:

state_name:active .targeted_class {{
    ...css style
}}

option states usage:

otp_state_name:key_name .targeted_class {{
    ...css style
}}

*/
",
        config.name()
    )
}
