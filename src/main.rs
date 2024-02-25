#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    battlesnake::run().into()
}
