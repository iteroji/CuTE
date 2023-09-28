/*
* Screen Menu Options
*
 */

use lazy_static::lazy_static;

lazy_static! {
    pub static ref REQUEST_MENU_OPTIONS: [&'static str; 9] = [
        "Add a URL\n \n",
        "Add Authentication\n \n",
        "Add Headers\n \n",
        "Enable verbose output\n \n",
        "Specify request output file\n \n",
        "Add Request Body\n \n",
        "Save this command\n \n",
        "Recursive download (wget only)\n \n",
        "Execute command\n \n",
    ];
    pub static ref METHOD_MENU_OPTIONS: [&'static str; 5] = [
        "GET\n \n",
        "POST\n \n",
        "PUT\n \n",
        "DELETE\n \n",
        "PATCH\n \n",
    ];
    pub static ref AUTHENTICATION_MENU_OPTIONS: [&'static str; 10] = [
        "Basic\n \n",
        "Bearer\n \n",
        "Digest\n \n",
        "Hawk\n \n",
        "OAuth\n \n",
        "AWS Signature\n \n",
        "NTLM\n \n",
        "Kerberos\n \n",
        "SPNEGO\n \n",
        "Custom\n \n",
    ];
    pub static ref INPUT_MENU_OPTIONS: [&'static str; 3] = [
        "Please enter a URL for your request",
        "Please specify your request headers",
        "Please enter your request body",
    ];
    pub static ref MAIN_MENU_OPTIONS: [&'static str; 5] = [
        "Build and run a new cURL command\n  \n",
        "Build and run a new wget command\n  \n",
        "Build/send new custom HTTP request\n  \n",
        "View my stored API keys\n  \n",
        "View or execute my saved commands\n  \n",
    ];
    pub static ref RESPONSE_MENU_OPTIONS: [&'static str; 3] = [
        "Write to file?\n \n",
        "View response headers\n \n",
        "View response body\n \n",
    ];
    pub static ref API_KEY_MENU_OPTIONS: [&'static str; 3] = [
        "Add a new key\n \n",
        "View my keys\n \n",
        "Delete a key\n \n",
    ];
    pub static ref SAVED_COMMAND_OPTIONS: [&'static str; 3] = [
        "Add a new saved command\n \n",
        "View my saved commands\n \n",
        "Delete a saved command\n \n",
    ];
}