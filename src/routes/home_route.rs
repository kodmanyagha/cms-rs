pub fn home_route() -> Router {
	Router::new().route("/", get(mainpage_handler))
}
