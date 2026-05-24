"""Tests for the github_mock_api Python bindings."""

import pytest
from github_mock_api import MockServer


class TestMockServer:
    @pytest.fixture
    def server(self) -> MockServer:
        _server = MockServer.start()
        yield _server
        _server.stop()

    @pytest.fixture
    def servers(self) -> list[MockServer]:
        _servers: list[MockServer] = []
        yield _servers
        for s in _servers:
            s.stop()

    def test_start(self, server: MockServer) -> None:
        uri = server.uri()
        assert uri.startswith("http://127.0.0.1:")

    def test_start_on(self, servers: list[MockServer]) -> None:
        server = MockServer.start_on("127.0.0.1", 0)
        servers.append(server)
        uri = server.uri()
        assert uri.startswith("http://127.0.0.1:")
        server.stop()

    def test_stop_is_idempotent(self, server: MockServer) -> None:
        server.stop()
        server.stop()

    def test_uri_accessible_after_stop(self, server: MockServer) -> None:
        uri = server.uri()
        server.stop()
        assert server.uri() == uri

    def test_start_on_invalid_host_raises_value_error(self) -> None:
        with pytest.raises(ValueError):
            MockServer.start_on("not-an-ip", 0)

    def test_multiple_servers(self, servers: list[MockServer]) -> None:
        server1 = MockServer.start()
        server2 = MockServer.start()
        servers.append(server1)
        servers.append(server2)
        assert server1.uri() != server2.uri()
        server1.stop()
        server2.stop()

    def test_start_on_exact_port(self, servers: list[MockServer]) -> None:
        server = MockServer.start_on("127.0.0.1", 12346)
        servers.append(server)
        assert server.uri() == "http://127.0.0.1:12346"
        server.stop()
