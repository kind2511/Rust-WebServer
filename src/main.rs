use axum::{
    extract::Path,                           // extract path segments like name in /greet/:name
    http::StatusCode,
    response::{IntoResponse, Response},     
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::fmt;

// Converts instances of Greet into JSON.
#[derive(Serialize)]
struct Greet {
    greet: String,
    name: String,
}

// Deserializes data from JSON back into instances of the GreetMe struct.
#[derive(Deserialize)]
struct GreetMe {
    input: String,
    name: String,
}

// Used for creating error messages
enum ServerError {
    MissingName,
    InvalidPayload,
}

// Allows us to custimize how instances of ServerError.
// Match is used to match for the different variants of ServerError.
// Depending on the error the corresponding error message is written to the formatter.
//The '_' indicates an anonymous lifetime, meaning it's not explicitly named, but it's there and tied to the scope of the function call.
impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::MissingName => write!(f, "Name parameter not provided."),
            ServerError::InvalidPayload => write!(f, "Invalid request payload."),
        }
    }
}

// IntoResponse is a trait that allows types to be converted into HTTP responses.
// Types that implement IntoResponse can be returned from handlers.
// Matches in the variant of the ServerError to determines the correct message and statuscode. 
// It then creates a tuple containg the stauscode and error message
impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ServerError::MissingName => (StatusCode::BAD_REQUEST, self.to_string()),
            ServerError::InvalidPayload => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        (status, error_message).into_response() // into_response converts it into a Response
    }
}


#[tokio::main] // allows the main function to be asynchronous. Gives main a tokio runtime
async fn main() {
    // Attempt to bind TCP listener
    let listener_result = tokio::net::TcpListener::bind("0.0.0.0:4000").await;
    // Check if binding was successful
    let listener = match listener_result {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Error binding listener: {}", err);
            return;
        }
    };

    // Create Axum app
    let app = Router::new()                     // creates router
        .route("/hello", get(hello))
        .route("/", get(not_found))
        .route("/greet/:name", get(greet_user))
        .route("/greetme", post(greet_user_post))
        .fallback(get(fallback));                       // handles request that does not match any route

    // Serve HTTP requests
    if let Err(err) = axum::serve(listener, app).await {
        eprintln!("Internal Server error: {}", err);
        return;
    }
}

// Statuscode returned is 200 by default
async fn hello() -> Result<&'static str, StatusCode> {
    Ok("Hello, World!")
}

// Function returns a type that implements the IntoResponse trait. 
// In Axum, the IntoResponse trait provides a way to convert types into HTTP responses.
// Axum provides implementations of the IntoResponse trait for certain tuple types, such as (StatusCode, &'static str)
// Axum provides automatic conversion of certain tuple types into HTTP responses through implementations of the IntoResponse trait.
async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 page not found")
}

// Takes a name as a param. 
// If name is empty returns a ServerError MissingName
// if name is there: creates a greet struct that is serialized into JSON
async fn greet_user(Path(name): Path<String>) -> Result<Json<Greet>, ServerError> {
    if name.is_empty() {
        return Err(ServerError::MissingName);
    }

    let greeting = Greet {
        greet: "Hello".to_string(),
        name,
    };
    Ok(Json(greeting))
}

// Takes in json as param, in the form of a Greetme struct
// If both input and are present return the greeting as json
// If one or both of the params of the payload is missing: return ServerError:InvalidPayload
async fn greet_user_post(Json(payload): Json<Option<GreetMe>>) -> Result<Json<Greet>, ServerError> {
    match payload {
        Some(body) if !body.input.is_empty() && !body.name.is_empty() => {
            let greeting = Greet {
                greet: body.input,
                name: body.name,
            };
            Ok(Json(greeting))
        },
        Some(_) => Err(ServerError::InvalidPayload),
        None => Err(ServerError::InvalidPayload),
    }
}

// If no route matches
async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Empty Response!")
}

