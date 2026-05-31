#include <memory>
#include <optional>
#include <string>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>

#define CATCH_CONFIG_MAIN
#include <catch2/catch.hpp>

#include "github_mock_api/MockServer.hpp"

using namespace github_mock_api_ffi;

namespace {

int connect_to(const std::string& host, int port) {
    int sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock < 0) return -1;

    struct sockaddr_in addr{};
    addr.sin_family = AF_INET;
    addr.sin_port = htons(port);
    if (inet_pton(AF_INET, host.c_str(), &addr.sin_addr) != 1) {
        close(sock);
        return -1;
    }

    int ret = connect(sock, (struct sockaddr*)&addr, sizeof(addr));
    close(sock);
    return ret;
}

std::pair<std::string, int> parse_uri(const std::string& uri) {
    auto host_part = uri.substr(7);
    auto colon_pos = host_part.rfind(':');
    auto host = host_part.substr(0, colon_pos);
    int port = std::stoi(host_part.substr(colon_pos + 1));
    return {host, port};
}

} // anonymous namespace

TEST_CASE("MockServer starts successfully", "[mock_server]") {
    auto result = MockServer::start();
    REQUIRE(result.is_ok());

    auto maybe_server = std::move(result).ok();
    REQUIRE(maybe_server.has_value());
}

TEST_CASE("MockServer URI format after default start", "[mock_server]") {
    auto result = MockServer::start();
    REQUIRE(result.is_ok());

    auto maybe_server = std::move(result).ok();
    REQUIRE(maybe_server.has_value());
    auto server = std::move(maybe_server.value());

    auto uri = server->uri();
    CHECK(uri.rfind("http://127.0.0.1:", 0) == 0);
}

TEST_CASE("MockServer stop succeeds", "[mock_server]") {
    auto result = MockServer::start();
    REQUIRE(result.is_ok());

    auto maybe_server = std::move(result).ok();
    REQUIRE(maybe_server.has_value());
    auto server = std::move(maybe_server.value());

    auto stop_result = server->stop();
    CHECK(stop_result.is_ok());
}

TEST_CASE("MockServer stop is idempotent", "[mock_server]") {
    auto result = MockServer::start();
    REQUIRE(result.is_ok());

    auto maybe_server = std::move(result).ok();
    REQUIRE(maybe_server.has_value());
    auto server = std::move(maybe_server.value());

    CHECK(server->stop().is_ok());
    CHECK(server->stop().is_ok());
    CHECK(server->stop().is_ok());
}

TEST_CASE("MockServer start_on with port 0 picks random port", "[mock_server]") {
    auto result = MockServer::start_on("127.0.0.1", 0);
    REQUIRE(result.is_ok());

    auto maybe_inner = std::move(result).ok();
    REQUIRE(maybe_inner.has_value());

    auto inner = std::move(maybe_inner.value());
    REQUIRE(inner.is_ok());

    auto maybe_server = std::move(inner).ok();
    REQUIRE(maybe_server.has_value());
    auto server = std::move(maybe_server.value());

    auto uri = server->uri();
    CHECK(uri.rfind("http://127.0.0.1:", 0) == 0);
}

TEST_CASE("MockServer start_on with specific port", "[mock_server]") {
    auto result = MockServer::start_on("127.0.0.1", 17893);
    REQUIRE(result.is_ok());

    auto maybe_inner = std::move(result).ok();
    REQUIRE(maybe_inner.has_value());

    auto inner = std::move(maybe_inner.value());
    REQUIRE(inner.is_ok());

    auto maybe_server = std::move(inner).ok();
    REQUIRE(maybe_server.has_value());
    auto server = std::move(maybe_server.value());

    auto uri = server->uri();
    CHECK(uri == "http://127.0.0.1:17893");
}

TEST_CASE("MockServer start_on with invalid host returns error", "[mock_server]") {
    auto result = MockServer::start_on("not_a_valid_host", 8080);
    REQUIRE(result.is_ok());

    auto maybe_inner = std::move(result).ok();
    REQUIRE(maybe_inner.has_value());

    auto inner = std::move(maybe_inner.value());
    REQUIRE(inner.is_err());

    auto maybe_err = std::move(inner).err();
    REQUIRE(maybe_err.has_value());
    CHECK(maybe_err.value() == MockServerError::InvalidHost);
}

TEST_CASE("MockServer is reachable via TCP after start", "[mock_server][network]") {
    auto result = MockServer::start();
    REQUIRE(result.is_ok());

    auto maybe_server = std::move(result).ok();
    REQUIRE(maybe_server.has_value());
    auto server = std::move(maybe_server.value());

    auto [host, port] = parse_uri(server->uri());
    CHECK(connect_to(host, port) == 0);
}

TEST_CASE("MockServer is not reachable after stop", "[mock_server][network]") {
    auto result = MockServer::start();
    REQUIRE(result.is_ok());

    auto maybe_server = std::move(result).ok();
    REQUIRE(maybe_server.has_value());
    auto server = std::move(maybe_server.value());

    auto [host, port] = parse_uri(server->uri());

    REQUIRE(server->stop().is_ok());

    CHECK(connect_to(host, port) < 0);
}
