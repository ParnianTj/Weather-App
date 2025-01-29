# Weather Notifier App
This is a weather notifier application which retrieves real-time data from AccuWeather API by searching for the specific city.

### Requirements before Running the App

- Rust installed [rustc 1.84.0] (https://www.rust-lang.org/tools/install)
- AccuWeather Account (https://developer.accuweather.com/user/register)
- Phlorin API Connecter (https://phlorin.com/api-connector/)

## How to Run this App?

1) Search for AccuWeather platform.
2) Create an account and get your API key.
3) Copy your API key and save for later.
4) Go to Google Sheets.
5) Click on Extensions and open Phlorin API Connecter.
6) Create a new API connector and select AccuWeather.
7) Enter your API key and save the connector.
8) Look for the city you wanted and copy the Loaction key.
9) Go back to the Rust code and replace the city with the Location key.
10) Run the code using `cargo run` command.

### How to customize the App?

You can always change the city that you chose with changing the Location key in the code and the time interval within which you want to get notifications from the app. 

By default, the app is set on Munich, Germany and a time interval of 2 minutes.

### Be aware of Traffic!

Sometimes if you had too many requests from AccuWeather API may give you this error: Failed to fetch weather data: error decoding response body. After you see that, please try again later or change your subscription package. Remember that in case you wanted to get your location key and check if the get request link is working, you can use Postman API Platfrom (https://www.postman.com/).
