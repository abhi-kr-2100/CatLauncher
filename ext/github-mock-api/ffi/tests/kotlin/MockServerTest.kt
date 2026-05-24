fun main() {
    try {
        val server = io.github.abhi_kr_2100.github_mock_api_ffi.MockServer.start().getOrThrow()

        val uri = server.uri()
        if (!uri.startsWith("http://127.0.0.1:")) {
            System.err.println("unexpected uri: $uri")
            try { server.stop() } catch (_: Exception) {}
            System.exit(1)
        }

        server.stop()
    } catch (e: Exception) {
        e.printStackTrace()
        System.err.println("MockServer test failed: $e")
        System.exit(1)
    }
}
