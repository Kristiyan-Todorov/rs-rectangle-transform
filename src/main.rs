use axum::{
    extract::Path,
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{any_service, get},
    Json, Router,
};
use generator::Generator;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub mod generator;
pub mod rectangle;

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Resource not found.")
}

async fn generate_rectangles(Path(number_of_rectangles): Path<usize>) -> Response {
    let generator = Generator::new(number_of_rectangles);

    if number_of_rectangles < 5 || number_of_rectangles > 15 {
        let res = (StatusCode::BAD_REQUEST, "Invalid input parameters");
        return res.into_response();
    }

    generator
        .write_file("rectangle_transform.json")
        .expect("unable to write to file");

    let res = Json(generator.generate_json());

    res.into_response()
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8090";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server listening on: http://{}", addr);

    axum::serve(
        listener,
        Router::new()
            .route(
                "/",
                any_service(
                    ServeDir::new("index.html").not_found_service(handle_404.into_service()),
                ),
            )
            .route(
                "/rectangles/:number_of_rectangles",
                get(generate_rectangles),
            ),
    )
    .await
    .unwrap();
}
