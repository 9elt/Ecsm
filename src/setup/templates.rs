use crate::config::ECSMConfig;

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

    <!-- link stylesheets from your \"src\" directory -->
    <link rel=\"stylesheet\" href=\"/css/main.css\">
</head>

<body>

    <div class=\"page-container\">

        <div>
            <!-- this is toggable -->
            <p class=\"btn\" handle_state=\"boolean\">toggle ligth (boolean)</p>
        </div>

        <div class=\"boolean-example\">
            <p>this is handled by the boolean state</p>
        </div>

        <div class=\"separator\"></div>

        <div>
            <!-- this is a selection -->
            <p class=\"btn\" handle_state=\"selection:light\">ligth on (selection)</p>

            <!-- this is selected by default, \":default\" is key -->
            <p class=\"btn\" handle_state=\"selection:default\">ligth off (selection)</p>
        </div>

        <div class=\"selection-example\">
            <p>this is handled by the selection state</p>
        </div>

    </div>

</body>

</html>    
",
        config.name()
    )
}

pub fn css_template(_config: &ECSMConfig) -> String {
    "\
html,
body {
    margin: 0;
    width: 100%;
    min-height: 100%;
}

body {
    color: #fff;
    background-image: linear-gradient(25deg, #111, #333);
    font-family: 'Segoe UI', 'Roboto', 'Oxygen', sans-serif;
}

.page-container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    min-height: 70vh;
}

div {
    text-align: center;
    max-width: 40rem;
    margin-inline: auto;
}

/* example usage */

.boolean-example,
.selection-example {
    padding-inline: 1rem;
    font-size: 1.4rem;
    border-radius: 4px;
    transition: all 100ms ease-in-out;
}

/*

boolean states can be selected as
\"state_name\":active

*/
boolean:active .boolean-example {
    color: #222;
    background-color: #69bdff;
    box-shadow: 0 0 6px #69bdff;
}

/* 

selection states can be selected as
\"state_name\":\"state_key\"

*/
selection:light .selection-example {
    color: #222;
    background-color: #ff6969;
    box-shadow: 0 0 6px #ff6969;
}

/* end example... */

.btn {
    padding: .25rem 1rem;
    background-color: #222;
    border-radius: .2rem;
    cursor: pointer;
}

.btn:hover {
    background-color: #111;
}

.separator {
    margin-block: 2rem;
    width: 100%;
    border-bottom: 2px solid #111;
}      
"
    .to_string()
}
